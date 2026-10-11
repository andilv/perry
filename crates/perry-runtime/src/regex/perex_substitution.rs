//! GetSubstitution, parsed once per replacement and emitted per match.
//!
//! The template is scanned as WTF-8 bytes. Every character GetSubstitution
//! recognises (`$`, `&`, `` ` ``, `'`, `<`, `>` and the digits) is ASCII, and
//! no ASCII byte occurs inside a multi-byte sequence, so the byte scan finds
//! exactly the markers a UTF-16 scan finds; the literal runs between them are
//! byte ranges of the template, emitted as they are.
use super::perex_api as api;
use super::perex_memory::StorageError;
use super::perex_output::{has_surrogate, wtf8_units, Output, Text};
use super::perex_runtime::EngineError;
use crate::gc::{RuntimeHandle, RuntimeHandleScope};
use crate::string::StringHeader;
use crate::value::{js_nanbox_string, TAG_UNDEFINED};
use perex::Budget;

/// One piece of a template.
#[derive(Clone, Copy)]
pub(super) enum Token {
    /// Bytes `start..end` of the template, holding `units` units.
    Literal(usize, usize, usize),
    Matched,
    Before,
    After,
    /// A capture, numbered from 1.
    Capture(usize),
    /// `$<name>`: the name's bytes in the template.
    Named(usize, usize),
}

/// Parse `template` (rooted heap string) for a match with `captures`
/// capture groups (group zero excluded). `named` says the match has a groups
/// object, so `$<name>` is a reference rather than literal text.
pub(super) fn parse(
    template: &RuntimeHandle<'_>,
    captures: usize,
    named: bool,
) -> Result<Vec<Token>, EngineError> {
    // Binding proves the payload is well-formed WTF-8 (and marks it), the
    // condition for scanning and copying it as bytes.
    api::bind_heap_subject(*template)?;
    unsafe { template.with_string_bytes(|bytes| scan(bytes, captures, named)) }
}

fn scan(bytes: &[u8], captures: usize, named: bool) -> Result<Vec<Token>, EngineError> {
    let n = bytes.len();
    let mut tokens = Vec::new();
    let (mut i, mut literal) = (0, 0);
    let digit = |b: u8| b.is_ascii_digit().then(|| (b - b'0') as usize);
    while i < n {
        if bytes[i] != b'$' || i + 1 == n {
            i += 1;
            continue;
        }
        let mut next = i + 2;
        let token = match bytes[i + 1] {
            b'$' => Token::Literal(i, i + 1, 1),
            b'&' => Token::Matched,
            b'`' => Token::Before,
            b'\'' => Token::After,
            b'0'..=b'9' => {
                let mut index = (bytes[i + 1] - b'0') as usize;
                if let Some(second) = bytes.get(next).copied().and_then(digit) {
                    let two = index * 10 + second;
                    if two > 0 && two <= captures {
                        index = two;
                        next += 1;
                    }
                }
                if index == 0 || index > captures {
                    i += 1;
                    continue;
                }
                Token::Capture(index)
            }
            b'<' if named => {
                let Some(close) = bytes[next..].iter().position(|&b| b == b'>') else {
                    i += 1;
                    continue;
                };
                let token = Token::Named(next, next + close);
                next += close + 1;
                token
            }
            _ => {
                i += 1;
                continue;
            }
        };
        tokens
            .try_reserve(2)
            .map_err(|_| StorageError::Allocation)?;
        if literal < i {
            tokens.push(Token::Literal(literal, i, wtf8_units(&bytes[literal..i])));
        }
        tokens.push(token);
        i = next;
        literal = i;
    }
    if literal < n {
        tokens
            .try_reserve(1)
            .map_err(|_| StorageError::Allocation)?;
        tokens.push(Token::Literal(literal, n, wtf8_units(&bytes[literal..n])));
    }
    Ok(tokens)
}

/// What a match supplies to its substitution, beyond spans of the subject.
pub(super) trait Parts {
    /// The matched substring's length, for `` $' ``.
    fn matched_len(&self) -> usize;
    /// Append the matched substring.
    fn matched(
        &mut self,
        input: &mut Text<'_, '_>,
        output: &mut Output,
        budget: &mut Budget,
    ) -> Result<(), EngineError>;
    /// Append capture `index` (from 1); an unset capture appends nothing.
    fn capture(
        &mut self,
        index: usize,
        input: &mut Text<'_, '_>,
        output: &mut Output,
        budget: &mut Budget,
    ) -> Result<(), EngineError>;
    /// The groups object, when the match has one.
    fn groups(&self) -> Option<&RuntimeHandle<'_>> {
        None
    }
}

/// Append the substitution `tokens` of `template` for one match at
/// `position` of `input`.
#[allow(clippy::too_many_arguments)]
pub(super) fn emit(
    tokens: &[Token],
    template: &RuntimeHandle<'_>,
    input: &mut Text<'_, '_>,
    position: usize,
    parts: &mut impl Parts,
    output: &mut Output,
    budget: &mut Budget,
) -> Result<(), EngineError> {
    for token in tokens {
        match *token {
            Token::Literal(start, end, units) => {
                output.bytes_of(template, start, end, units, budget)?
            }
            Token::Matched => parts.matched(input, output, budget)?,
            Token::Before => output.span(input, 0, position.min(input.len()), budget)?,
            Token::After => {
                let length = input.len();
                let start = position
                    .checked_add(parts.matched_len())
                    .ok_or(StorageError::Limit)?
                    .min(length);
                output.span(input, start, length, budget)?
            }
            Token::Capture(index) => parts.capture(index, input, output, budget)?,
            Token::Named(start, end) => {
                let Some(groups) = parts.groups() else {
                    return Err(EngineError::InvalidSpan);
                };
                let scope = RuntimeHandleScope::new();
                let key = name(template, start, end)?;
                let key = scope.root_string_ptr(key);
                let value = api::caught(|| {
                    let key =
                        key.with_const_ptr::<StringHeader, _>(|key| js_nanbox_string(key as i64));
                    crate::proxy::js_reflect_get(
                        groups.get_nanbox_f64(),
                        key,
                        groups.get_nanbox_f64(),
                    )
                })?;
                let value = scope.root_nanbox_f64(value);
                if value.get_nanbox_f64().to_bits() != TAG_UNDEFINED {
                    output.value(&value, budget)?;
                }
            }
        }
    }
    Ok(())
}

/// The group name `start..end` (bytes) of `template` as a new string.
fn name(
    template: &RuntimeHandle<'_>,
    start: usize,
    end: usize,
) -> Result<*mut StringHeader, EngineError> {
    // Copied out first: the allocation below may move the template.
    let bytes =
        unsafe { template.with_string_bytes(|bytes| bytes.get(start..end).map(<[u8]>::to_vec)) }
            .ok_or(EngineError::InvalidSpan)?;
    let units = wtf8_units(&bytes);
    let flags = if has_surrogate(&bytes) {
        crate::string::STRING_FLAG_HAS_LONE_SURROGATES
    } else {
        0
    };
    Ok(crate::string::js_string_from_bytes_known_utf16(
        bytes.as_ptr(),
        bytes.len() as u32,
        units as u32,
        flags,
    ))
}

#![allow(dead_code)]

use crate::err::AssemblyError;
use nom::IResult;
use nom_locate::LocatedSpan;

mod space;
mod cinstr;
mod parsing;
mod util;

pub type Span<'a> = LocatedSpan<&'a str>;
type PResult<'a, I> = IResult<Span<'a>, I, AssemblyError>;

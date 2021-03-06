/*
Copyright (C) 2018-2019 de4dot@gmail.com

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use super::super::super::iced_constants::IcedConstants;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
enum MemoryKeywords {
	None,
	byte_ptr,
	dword_bcst,
	dword_ptr,
	fpuenv14_ptr,
	fpuenv28_ptr,
	fpustate108_ptr,
	fpustate94_ptr,
	fword_ptr,
	oword_ptr,
	qword_bcst,
	qword_ptr,
	tbyte_ptr,
	word_ptr,
	xmmword_ptr,
	ymmword_ptr,
	zmmword_ptr,
	mem384_ptr,
}

// GENERATOR-BEGIN: MemorySizes
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static MEM_SIZE_TBL_DATA: [MemoryKeywords; 141] = [
	MemoryKeywords::None,
	MemoryKeywords::byte_ptr,
	MemoryKeywords::word_ptr,
	MemoryKeywords::dword_ptr,
	MemoryKeywords::qword_ptr,
	MemoryKeywords::qword_ptr,
	MemoryKeywords::xmmword_ptr,
	MemoryKeywords::ymmword_ptr,
	MemoryKeywords::zmmword_ptr,
	MemoryKeywords::byte_ptr,
	MemoryKeywords::word_ptr,
	MemoryKeywords::dword_ptr,
	MemoryKeywords::qword_ptr,
	MemoryKeywords::xmmword_ptr,
	MemoryKeywords::ymmword_ptr,
	MemoryKeywords::zmmword_ptr,
	MemoryKeywords::dword_ptr,
	MemoryKeywords::fword_ptr,
	MemoryKeywords::tbyte_ptr,
	MemoryKeywords::word_ptr,
	MemoryKeywords::dword_ptr,
	MemoryKeywords::qword_ptr,
	MemoryKeywords::dword_ptr,
	MemoryKeywords::qword_ptr,
	MemoryKeywords::qword_ptr,
	MemoryKeywords::oword_ptr,
	MemoryKeywords::fword_ptr,
	MemoryKeywords::fword_ptr,
	MemoryKeywords::word_ptr,
	MemoryKeywords::dword_ptr,
	MemoryKeywords::qword_ptr,
	MemoryKeywords::tbyte_ptr,
	MemoryKeywords::xmmword_ptr,
	MemoryKeywords::word_ptr,
	MemoryKeywords::fpuenv14_ptr,
	MemoryKeywords::fpuenv28_ptr,
	MemoryKeywords::fpustate94_ptr,
	MemoryKeywords::fpustate108_ptr,
	MemoryKeywords::None,
	MemoryKeywords::None,
	MemoryKeywords::None,
	MemoryKeywords::None,
	MemoryKeywords::tbyte_ptr,
	MemoryKeywords::None,
	MemoryKeywords::None,
	MemoryKeywords::tbyte_ptr,
	MemoryKeywords::mem384_ptr,
	MemoryKeywords::xmmword_ptr,
	MemoryKeywords::word_ptr,
	MemoryKeywords::word_ptr,
	MemoryKeywords::dword_ptr,
	MemoryKeywords::dword_ptr,
	MemoryKeywords::dword_ptr,
	MemoryKeywords::dword_ptr,
	MemoryKeywords::dword_ptr,
	MemoryKeywords::qword_ptr,
	MemoryKeywords::qword_ptr,
	MemoryKeywords::qword_ptr,
	MemoryKeywords::qword_ptr,
	MemoryKeywords::qword_ptr,
	MemoryKeywords::qword_ptr,
	MemoryKeywords::qword_ptr,
	MemoryKeywords::qword_ptr,
	MemoryKeywords::xmmword_ptr,
	MemoryKeywords::xmmword_ptr,
	MemoryKeywords::xmmword_ptr,
	MemoryKeywords::xmmword_ptr,
	MemoryKeywords::xmmword_ptr,
	MemoryKeywords::xmmword_ptr,
	MemoryKeywords::xmmword_ptr,
	MemoryKeywords::xmmword_ptr,
	MemoryKeywords::xmmword_ptr,
	MemoryKeywords::xmmword_ptr,
	MemoryKeywords::xmmword_ptr,
	MemoryKeywords::xmmword_ptr,
	MemoryKeywords::xmmword_ptr,
	MemoryKeywords::ymmword_ptr,
	MemoryKeywords::ymmword_ptr,
	MemoryKeywords::ymmword_ptr,
	MemoryKeywords::ymmword_ptr,
	MemoryKeywords::ymmword_ptr,
	MemoryKeywords::ymmword_ptr,
	MemoryKeywords::ymmword_ptr,
	MemoryKeywords::ymmword_ptr,
	MemoryKeywords::ymmword_ptr,
	MemoryKeywords::ymmword_ptr,
	MemoryKeywords::ymmword_ptr,
	MemoryKeywords::ymmword_ptr,
	MemoryKeywords::ymmword_ptr,
	MemoryKeywords::ymmword_ptr,
	MemoryKeywords::ymmword_ptr,
	MemoryKeywords::ymmword_ptr,
	MemoryKeywords::zmmword_ptr,
	MemoryKeywords::zmmword_ptr,
	MemoryKeywords::zmmword_ptr,
	MemoryKeywords::zmmword_ptr,
	MemoryKeywords::zmmword_ptr,
	MemoryKeywords::zmmword_ptr,
	MemoryKeywords::zmmword_ptr,
	MemoryKeywords::zmmword_ptr,
	MemoryKeywords::zmmword_ptr,
	MemoryKeywords::zmmword_ptr,
	MemoryKeywords::zmmword_ptr,
	MemoryKeywords::zmmword_ptr,
	MemoryKeywords::zmmword_ptr,
	MemoryKeywords::dword_bcst,
	MemoryKeywords::dword_bcst,
	MemoryKeywords::dword_bcst,
	MemoryKeywords::dword_bcst,
	MemoryKeywords::dword_bcst,
	MemoryKeywords::qword_bcst,
	MemoryKeywords::qword_bcst,
	MemoryKeywords::qword_bcst,
	MemoryKeywords::dword_bcst,
	MemoryKeywords::qword_bcst,
	MemoryKeywords::dword_bcst,
	MemoryKeywords::dword_bcst,
	MemoryKeywords::qword_bcst,
	MemoryKeywords::qword_bcst,
	MemoryKeywords::qword_bcst,
	MemoryKeywords::dword_bcst,
	MemoryKeywords::qword_bcst,
	MemoryKeywords::dword_bcst,
	MemoryKeywords::dword_bcst,
	MemoryKeywords::qword_bcst,
	MemoryKeywords::qword_bcst,
	MemoryKeywords::qword_bcst,
	MemoryKeywords::dword_bcst,
	MemoryKeywords::qword_bcst,
	MemoryKeywords::dword_bcst,
	MemoryKeywords::dword_bcst,
	MemoryKeywords::dword_bcst,
	MemoryKeywords::qword_bcst,
	MemoryKeywords::qword_bcst,
	MemoryKeywords::qword_bcst,
	MemoryKeywords::qword_bcst,
	MemoryKeywords::qword_bcst,
	MemoryKeywords::qword_bcst,
	MemoryKeywords::dword_bcst,
	MemoryKeywords::dword_bcst,
	MemoryKeywords::dword_bcst,
];
// GENERATOR-END: MemorySizes

lazy_static! {
	pub(super) static ref MEM_SIZE_TBL: Vec<&'static str> = {
		let mut v = Vec::with_capacity(IcedConstants::NUMBER_OF_MEMORY_SIZES);
		for &mem_keywords in MEM_SIZE_TBL_DATA.iter() {
			let keywords: &'static str = match mem_keywords {
				MemoryKeywords::None => "",
				MemoryKeywords::byte_ptr => "byte ptr ",
				MemoryKeywords::dword_bcst => "dword bcst ",
				MemoryKeywords::dword_ptr => "dword ptr ",
				MemoryKeywords::fpuenv14_ptr => "fpuenv14 ptr ",
				MemoryKeywords::fpuenv28_ptr => "fpuenv28 ptr ",
				MemoryKeywords::fpustate108_ptr => "fpustate108 ptr ",
				MemoryKeywords::fpustate94_ptr => "fpustate94 ptr ",
				MemoryKeywords::fword_ptr => "fword ptr ",
				MemoryKeywords::oword_ptr => "oword ptr ",
				MemoryKeywords::qword_bcst => "qword bcst ",
				MemoryKeywords::qword_ptr => "qword ptr ",
				MemoryKeywords::tbyte_ptr => "tbyte ptr ",
				MemoryKeywords::word_ptr => "word ptr ",
				MemoryKeywords::xmmword_ptr => "xmmword ptr ",
				MemoryKeywords::ymmword_ptr => "ymmword ptr ",
				MemoryKeywords::zmmword_ptr => "zmmword ptr ",
				MemoryKeywords::mem384_ptr => "mem384 ptr ",
			};
			v.push(keywords);
		}
		v
	};
}

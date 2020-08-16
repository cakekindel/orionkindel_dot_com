; ModuleID = 'probe1.3a1fbbbh-cgu.0'
source_filename = "probe1.3a1fbbbh-cgu.0"
target datalayout = "e-m:e-p:32:32-i64:64-n32:64-S128"
target triple = "wasm32-unknown-unknown"

%"core::iter::adapters::Rev<core::iter::adapters::StepBy<core::ops::range::Range<i32>>>" = type { [0 x i32], %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>", [0 x i32] }
%"core::iter::adapters::StepBy<core::ops::range::Range<i32>>" = type { [0 x i32], { i32, i32 }, [0 x i32], i32, [0 x i8], i8, [3 x i8] }
%"core::panic::Location" = type { [0 x i32], { [0 x i8]*, i32 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }

@alloc1 = private unnamed_addr constant <{ [27 x i8] }> <{ [27 x i8] c"assertion failed: step != 0" }>, align 1
@alloc2 = private unnamed_addr constant <{ [73 x i8] }> <{ [73 x i8] c"/rustc/d3fb005a39e62501b8b0b356166e515ae24e2e54/src/libcore/macros/mod.rs" }>, align 1
@alloc3 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [73 x i8] }>, <{ [73 x i8] }>* @alloc2, i32 0, i32 0, i32 0), [12 x i8] c"I\00\00\00\0A\00\00\00\09\00\00\00" }>, align 4

; core::iter::traits::iterator::Iterator::rev
; Function Attrs: inlinehint nounwind
define hidden void @_ZN4core4iter6traits8iterator8Iterator3rev17h7dbab9075f819785E(%"core::iter::adapters::Rev<core::iter::adapters::StepBy<core::ops::range::Range<i32>>>"* noalias nocapture sret dereferenceable(16) %0, %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* noalias nocapture dereferenceable(16) %self) unnamed_addr #0 {
start:
  %_2 = alloca %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>", align 4
  %1 = bitcast %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* %_2 to i8*
  %2 = bitcast %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* %self to i8*
  call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 4 %1, i8* align 4 %2, i32 16, i1 false)
; call core::iter::adapters::Rev<T>::new
  call void @"_ZN4core4iter8adapters12Rev$LT$T$GT$3new17h8195489f0ca5aa22E"(%"core::iter::adapters::Rev<core::iter::adapters::StepBy<core::ops::range::Range<i32>>>"* noalias nocapture sret dereferenceable(16) %0, %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* noalias nocapture dereferenceable(16) %_2)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::iter::traits::iterator::Iterator::step_by
; Function Attrs: inlinehint nounwind
define hidden void @_ZN4core4iter6traits8iterator8Iterator7step_by17h457d7a91ae9084edE(%"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* noalias nocapture sret dereferenceable(16) %0, i32 %self.0, i32 %self.1, i32 %step) unnamed_addr #0 {
start:
; call core::iter::adapters::StepBy<I>::new
  call void @"_ZN4core4iter8adapters15StepBy$LT$I$GT$3new17he14556a84db6480dE"(%"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* noalias nocapture sret dereferenceable(16) %0, i32 %self.0, i32 %self.1, i32 %step)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::iter::adapters::Rev<T>::new
; Function Attrs: nounwind
define hidden void @"_ZN4core4iter8adapters12Rev$LT$T$GT$3new17h8195489f0ca5aa22E"(%"core::iter::adapters::Rev<core::iter::adapters::StepBy<core::ops::range::Range<i32>>>"* noalias nocapture sret dereferenceable(16) %0, %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* noalias nocapture dereferenceable(16) %iter) unnamed_addr #1 {
start:
  %_2 = alloca %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>", align 4
  %1 = bitcast %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* %_2 to i8*
  %2 = bitcast %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* %iter to i8*
  call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 4 %1, i8* align 4 %2, i32 16, i1 false)
  %3 = bitcast %"core::iter::adapters::Rev<core::iter::adapters::StepBy<core::ops::range::Range<i32>>>"* %0 to %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"*
  %4 = bitcast %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* %3 to i8*
  %5 = bitcast %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* %_2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 4 %4, i8* align 4 %5, i32 16, i1 false)
  ret void
}

; core::iter::adapters::StepBy<I>::new
; Function Attrs: nounwind
define hidden void @"_ZN4core4iter8adapters15StepBy$LT$I$GT$3new17he14556a84db6480dE"(%"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* noalias nocapture sret dereferenceable(16) %0, i32 %iter.0, i32 %iter.1, i32 %step) unnamed_addr #1 {
start:
  %_4 = icmp ne i32 %step, 0
  %_3 = xor i1 %_4, true
  br i1 %_3, label %bb2, label %bb1

bb1:                                              ; preds = %start
  %_9 = sub i32 %step, 1
  %1 = bitcast %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* %0 to { i32, i32 }*
  %2 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %1, i32 0, i32 0
  store i32 %iter.0, i32* %2, align 4
  %3 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %1, i32 0, i32 1
  store i32 %iter.1, i32* %3, align 4
  %4 = getelementptr inbounds %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>", %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* %0, i32 0, i32 3
  store i32 %_9, i32* %4, align 4
  %5 = getelementptr inbounds %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>", %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* %0, i32 0, i32 5
  store i8 1, i8* %5, align 4
  ret void

bb2:                                              ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h41106b821366e1d1E([0 x i8]* noalias nonnull readonly align 1 bitcast (<{ [27 x i8] }>* @alloc1 to [0 x i8]*), i32 27, %"core::panic::Location"* noalias readonly align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc3 to %"core::panic::Location"*))
  unreachable
}

; probe1::probe
; Function Attrs: nounwind
define hidden void @_ZN6probe15probe17he203a3b61d9abeacE() unnamed_addr #1 {
start:
  %_3 = alloca { i32, i32 }, align 4
  %_2 = alloca %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>", align 4
  %_1 = alloca %"core::iter::adapters::Rev<core::iter::adapters::StepBy<core::ops::range::Range<i32>>>", align 4
  %0 = bitcast { i32, i32 }* %_3 to i32*
  store i32 0, i32* %0, align 4
  %1 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %_3, i32 0, i32 1
  store i32 10, i32* %1, align 4
  %2 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %_3, i32 0, i32 0
  %3 = load i32, i32* %2, align 4
  %4 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %_3, i32 0, i32 1
  %5 = load i32, i32* %4, align 4
; call core::iter::traits::iterator::Iterator::step_by
  call void @_ZN4core4iter6traits8iterator8Iterator7step_by17h457d7a91ae9084edE(%"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* noalias nocapture sret dereferenceable(16) %_2, i32 %3, i32 %5, i32 2)
  br label %bb1

bb1:                                              ; preds = %start
; call core::iter::traits::iterator::Iterator::rev
  call void @_ZN4core4iter6traits8iterator8Iterator3rev17h7dbab9075f819785E(%"core::iter::adapters::Rev<core::iter::adapters::StepBy<core::ops::range::Range<i32>>>"* noalias nocapture sret dereferenceable(16) %_1, %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* noalias nocapture dereferenceable(16) %_2)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret void
}

; Function Attrs: argmemonly nounwind willreturn
declare void @llvm.memcpy.p0i8.p0i8.i32(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i32, i1 immarg) #2

; core::panicking::panic
; Function Attrs: cold noinline noreturn nounwind
declare void @_ZN4core9panicking5panic17h41106b821366e1d1E([0 x i8]* noalias nonnull readonly align 1, i32, %"core::panic::Location"* noalias readonly align 4 dereferenceable(16)) unnamed_addr #3

attributes #0 = { inlinehint nounwind "target-cpu"="generic" }
attributes #1 = { nounwind "target-cpu"="generic" }
attributes #2 = { argmemonly nounwind willreturn }
attributes #3 = { cold noinline noreturn nounwind "target-cpu"="generic" }

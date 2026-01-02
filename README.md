# ILP Data Preparation Experiment (Rust)

## Final Results (40,000,000 element)

| Method | Execution Time | Throughput | Strategy |
| :--- | :--- | :--- | :--- |
| **Native (`.sum()`)** | 47.36 ms | ~3.37 GB/s | Sequential Dependency (1 ALU) |
| **ILP Prepped (Manual)** | 12.55 ms | **~12.74 GB/s** | 4 Independent Accumulators |
| **ILP Prepped (Idiomatic)**| 12.90 ms | **~12.40 GB/s** | `chunks_exact` + Array Fold |

---

## Hardware Specs & Run
- **CPU**: Intel Core i5-7200U (Skylake) @ 2.50GHz
- **Dataset**: 160 MB (40M `f32` elements)

```bash
RUSTFLAGS="-C opt-level=3 -C target-cpu=native" cargo bench
```

## ASM Code

```asm
native_sum:
        test    rsi, rsi
        je      .LBB0_1
        mov     eax, esi
        and     eax, 7
        cmp     rsi, 8
        jae     .LBB0_4
        vxorps  xmm0, xmm0, xmm0
        xor     ecx, ecx
        jmp     .LBB0_6
.LBB0_1:
        vxorps  xmm0, xmm0, xmm0
        ret
.LBB0_4:
        and     rsi, -8
        vxorps  xmm0, xmm0, xmm0
        xor     ecx, ecx
.LBB0_5:
        vaddss  xmm0, xmm0, dword ptr [rdi + 4*rcx]
        vaddss  xmm0, xmm0, dword ptr [rdi + 4*rcx + 4]
        vaddss  xmm0, xmm0, dword ptr [rdi + 4*rcx + 8]
        vaddss  xmm0, xmm0, dword ptr [rdi + 4*rcx + 12]
        vaddss  xmm0, xmm0, dword ptr [rdi + 4*rcx + 16]
        vaddss  xmm0, xmm0, dword ptr [rdi + 4*rcx + 20]
        vaddss  xmm0, xmm0, dword ptr [rdi + 4*rcx + 24]
        vaddss  xmm0, xmm0, dword ptr [rdi + 4*rcx + 28]
        add     rcx, 8
        cmp     rsi, rcx
        jne     .LBB0_5
.LBB0_6:
        test    rax, rax
        je      .LBB0_9
        lea     rcx, [rdi + 4*rcx]
        xor     edx, edx
.LBB0_8:
        vaddss  xmm0, xmm0, dword ptr [rcx + 4*rdx]
        inc     rdx
        cmp     rax, rdx
        jne     .LBB0_8
.LBB0_9:
        ret

vliw_style_sum:
        mov     rax, rsi
        and     rax, -4
        je      .LBB1_1
        lea     r8, [rsi - 4]
        mov     ecx, r8d
        not     ecx
        test    cl, 28
        jne     .LBB1_4
        vxorps  xmm0, xmm0, xmm0
        mov     rcx, rax
        mov     rdx, rdi
        jmp     .LBB1_6
.LBB1_1:
        vxorps  xmm0, xmm0, xmm0
        shl     esi, 2
        and     esi, 12
        jne     .LBB1_11
        jmp     .LBB1_18
.LBB1_4:
        mov     r9d, r8d
        shr     r9d, 2
        inc     r9d
        and     r9d, 7
        vxorps  xmm0, xmm0, xmm0
        mov     rcx, rax
        mov     rdx, rdi
.LBB1_5:
        add     rcx, -4
        vaddps  xmm0, xmm0, xmmword ptr [rdx]
        add     rdx, 16
        dec     r9
        jne     .LBB1_5
.LBB1_6:
        cmp     r8, 28
        jb      .LBB1_9
        xor     r8d, r8d
.LBB1_8:
        vaddps  xmm0, xmm0, xmmword ptr [rdx + 4*r8]
        vaddps  xmm0, xmm0, xmmword ptr [rdx + 4*r8 + 16]
        vaddps  xmm0, xmm0, xmmword ptr [rdx + 4*r8 + 32]
        vaddps  xmm0, xmm0, xmmword ptr [rdx + 4*r8 + 48]
        vaddps  xmm0, xmm0, xmmword ptr [rdx + 4*r8 + 64]
        vaddps  xmm0, xmm0, xmmword ptr [rdx + 4*r8 + 80]
        vaddps  xmm0, xmm0, xmmword ptr [rdx + 4*r8 + 96]
        vaddps  xmm0, xmm0, xmmword ptr [rdx + 4*r8 + 112]
        add     r8, 32
        cmp     rcx, r8
        jne     .LBB1_8
.LBB1_9:
        vmovshdup       xmm1, xmm0
        vaddss  xmm1, xmm1, xmm0
        vshufpd xmm2, xmm0, xmm0, 1
        vaddss  xmm1, xmm2, xmm1
        vshufps xmm0, xmm0, xmm0, 255
        vaddss  xmm0, xmm0, xmm1
        shl     esi, 2
        and     esi, 12
        je      .LBB1_18
.LBB1_11:
        lea     rax, [rdi + 4*rax]
        lea     rdx, [rsi - 4]
        mov     ecx, edx
        not     ecx
        test    cl, 28
        jne     .LBB1_13
        mov     rcx, rax
        jmp     .LBB1_15
.LBB1_13:
        mov     edi, edx
        shr     edi, 2
        inc     edi
        and     edi, 7
        mov     rcx, rax
.LBB1_14:
        vaddss  xmm0, xmm0, dword ptr [rcx]
        add     rcx, 4
        dec     rdi
        jne     .LBB1_14
.LBB1_15:
        cmp     rdx, 28
        jb      .LBB1_18
        add     rax, rsi
.LBB1_17:
        vaddss  xmm0, xmm0, dword ptr [rcx]
        vaddss  xmm0, xmm0, dword ptr [rcx + 4]
        vaddss  xmm0, xmm0, dword ptr [rcx + 8]
        vaddss  xmm0, xmm0, dword ptr [rcx + 12]
        vaddss  xmm0, xmm0, dword ptr [rcx + 16]
        vaddss  xmm0, xmm0, dword ptr [rcx + 20]
        vaddss  xmm0, xmm0, dword ptr [rcx + 24]
        vaddss  xmm0, xmm0, dword ptr [rcx + 28]
        add     rcx, 32
        cmp     rcx, rax
        jne     .LBB1_17
.LBB1_18:
        ret

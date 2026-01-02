# Handled Data Preparation vs Native Data Preparation (for CPU)

In this benchmark, we verify Instruction-Level Parallelism (ILP) by unrolling the summation of a floating-point array into multiple independent accumulators. By breaking the data into separate chunks, we provide the processor's scheduler with independent instructions. The hardware scheduler recognizes that these operations have no data dependencies. It decomposes these instructions into a stream of micro-operations (uOps) and dispatches them to multiple execution ports simultaneously. This allows the CPU to sustain a throughput of multiple instructions per clock cycle (superscalar execution), effectively bypassing the bottleneck of the "one-instruction-per-cycle" serial execution model.

## **CPU Pipeline Flow (Instruction Level)**:

**RAM/Cache** → **Fetch** (seq.) → **Decoders** (paral.) → **Dispatch/Rename** (seq.) → **Reservation Station** (paral. IF independent uOps OR seq. IF data-dependent) → **Ports** (paral.) → **Retire** (seq.)

## Final Results of sum for [f32] (40M elements)

| Method | Execution Time | Throughput | Strategy |
| :--- | :--- | :--- | :--- |
| **Native (`.sum()`)** | 47.36 ms | ~3.37 GB/s | Sequential Dependency (1 ALU) |
| **Prepped** | 12.55 ms | **~12.74 GB/s** | 4 Independent Accumulators |

## Hardware Specs & Run
- **CPU**: Intel Core i5-7200U (Skylake) @ 2.50GHz
- **RAM**: 8GB DDR4-2400 (Single Channel)
- **Dataset**: 160 MB (40M `f32` elements)

```bash
RUSTFLAGS="-C opt-level=3 -C target-cpu=native" cargo bench
```

## ASM Code

```asm
prepped_sum:
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

.LCPI2_0:
        .long   0x80000000
sum:
        test    rsi, rsi
        je      .LBB2_1
        mov     eax, esi
        and     eax, 7
        cmp     rsi, 8
        jae     .LBB2_4
        vmovss  xmm0, dword ptr [rip + .LCPI2_0]
        xor     ecx, ecx
        jmp     .LBB2_6
.LBB2_1:
        vmovss  xmm0, dword ptr [rip + .LCPI2_0]
        ret
.LBB2_4:
        and     rsi, -8
        vmovss  xmm0, dword ptr [rip + .LCPI2_0]
        xor     ecx, ecx
.LBB2_5:
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
        jne     .LBB2_5
.LBB2_6:
        test    rax, rax
        je      .LBB2_9
        lea     rcx, [rdi + 4*rcx]
        xor     edx, edx
.LBB2_8:
        vaddss  xmm0, xmm0, dword ptr [rcx + 4*rdx]
        inc     rdx
        cmp     rax, rdx
        jne     .LBB2_8
.LBB2_9:
        ret


   
       
.LBB1_18:
        ret

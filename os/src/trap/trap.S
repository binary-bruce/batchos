.altmacro

# save register value to stack
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm

# load the stack value to register
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm

    .section .text
    .globl __alltraps
    .globl __restore
    .align 2

# __alltraps is store in stvec register: see trap.rs init function
# ecall triggers calling __alltraps by:
# 1. save pc in sepc
# 2. raise privilege to supervisor mode
# 3. jump to stvec
__alltraps:
    # see __restore: `csrrw sp, sscratch, sp` which put kernel stack value to sscratch
    csrrw sp, sscratch, sp # now sp->kernel stack, sscratch->user stack
    
    # allocate a TrapContext on kernel stack
    addi sp, sp, -34*8

    # save general-purpose registers
    sd x1, 1*8(sp)
    # skip sp(x2), we will save it later
    sd x3, 3*8(sp)
    # skip tp(x4), application does not use it
    # save x5~x31
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr

    # save sstatus and sepc to trap context
    # we can use t0/t1/t2 freely, because they were saved on kernel stack
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)

    # read user stack from sscratch and save it on the kernel stack
    csrr t2, sscratch
    sd t2, 2*8(sp)

    # set input argument of trap_handler(cx: &mut TrapContext)
    mv a0, sp # sp points to pushed trap context
    call trap_handler

# Switch from supervisor mode to user mode
# case1: start running app by __restore
# case2: back to U after handling trap
__restore:
    # a0 is the address of trap context, see batch.rs: `__restore(ctx as *const _ as usize);`
    # the address of trap context is actually the kernel stack after pushing the trap context to kernel stack
    mv sp, a0 # now sp->kernel stack(after allocated)
    
    # sstatus in trap context, it has information of the previous privilege which is set to user mode
    ld t0, 32*8(sp)

     # sepc in trap context, the entry point of user mode
    ld t1, 33*8(sp)

    # user stack
    ld t2, 2*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    csrw sscratch, t2 # sscratch->user stack
    
    # restore general-purpuse registers except sp/tp
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr

    # release TrapContext on kernel stack
    addi sp, sp, 34*8

    # now sp->user stack, sscratch->kernel stack
    csrrw sp, sscratch, sp

    # switch to user mode from supervisor mode
    sret

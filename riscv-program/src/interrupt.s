.global interrupt

interrupt:
    ecall;
    jalr x0, x1, 0;

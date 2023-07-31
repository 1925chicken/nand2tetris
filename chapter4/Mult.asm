    @R1
    D=M
    @n
    M=D
    @R2
    M=0
(LOOP)
    @n
    D=M
    @END
    D; JEQ
    @R0
    D=M
    @R2
    M=M+D
    @n
    M=M-1
    @LOOP
    0; JMP
(END)
    @END
    0; JMP

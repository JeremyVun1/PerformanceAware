bits 16

mov bp, word [0]

; cols, rows
; mov cx, 2
mov dx, 100

; rgb values
mov ax, 0
mov bx, 0

mov si, 0

col_loop:
	mov cx, 100

	row_loop:
		mov word [bp], ax ; rgb
		mov word [bp + 2], bx ; g
		mov word [bp + 3], 65535; alpha
		add ax, 10
		add bx, 10
		add bp, 4

		sub cx, 1
		jnz row_loop

	sub dx, 1
	jnz col_loop

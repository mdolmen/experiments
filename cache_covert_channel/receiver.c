#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <unistd.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <pthread.h>
#include <dlfcn.h>
#include <stdarg.h>
#include <signal.h>

#include <math.h>

#define THRESHOLD 100

#define LIBMATH_PATH "/lib64/libm.so.6"
#define LIBMATH_ASIN "asin"
#define LIBMATH_ACOS "acos"
#define LIBMATH_FMODF "fmodf"

#define LIBMATH_ASIN_OFFSET 0x42
#define LIBMATH_ACOS_OFFSET 0x42
//#define LIBMATH_FMODF_OFFSET 0xa

/*
 * the version of lgamma called depends on the glibc version, so its better to
 * use another symbol instead.
 * <lgammaf@@GLIBC_2.23>
 * <lgammaf@@GLIBC_2.2.5>
 */

// closest exported symbol is lgammaf
#define LIBMATH_LGAMMAF "lgammaf"
// annobin_w_expf_compat.c_end
//#define LIBMATH_LGAMMAF_OFFSET 0x7
// annobin_k_standardf.c_end
#define LIBMATH_LGAMMAF_OFFSET 0x13e
#define LIBMATH_TGAMMAF "tgammaf"
#define LIBMATH_TGAMMAF_OFFSET 0x54e

#define LIBMATH_FMODF "fmodf"
// annobin_w_exp10f_compat.c_end
#define LIBMATH_FMODF_OFFSET 4

#define RESULTS_SIZE 1024*1024

unsigned char *results;
pthread_mutex_t stopMutex;
int stop_probing = 0;

/* utils functions */
static void error(const char * format, ...) {
	va_list myargs;
	va_start(myargs, format);
	printf("[\033[31;1m!\033[0m] ");
	vprintf(format, myargs);
	printf("\n");
	exit(1);
}
static void info(const char * format, ...) {
	va_list myargs;
	va_start(myargs, format);
	printf("[\033[34;1m-\033[0m] ");
	vprintf(format, myargs);
	printf("\n");
}
static void ok(const char * format, ...) {
	va_list myargs;
	va_start(myargs, format);
	printf("[\033[32;1m+\033[0m] ");
	vprintf(format, myargs);
	printf("\n");
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

// TODO: change addresses
/* Probing thread */
void *probe_thread(void *arg) {
	char *dl_error;
	void *library = dlopen(LIBMATH_PATH, RTLD_NOW);
	if (!library) {
		error("dlopen failed: %s",dl_error);
	}

	void *asin = dlsym(library, LIBMATH_ASIN);
	if ((dl_error = dlerror()) != NULL)  {
		error("error in dlsym : %s",dl_error);
	}

	void *acos = dlsym(library, LIBMATH_ACOS);
	if ((dl_error = dlerror()) != NULL)  {
		error("error in dlsym : %s",dl_error);
	}

	void *lgammaf = dlsym(library, LIBMATH_LGAMMAF);
	if ((dl_error = dlerror()) != NULL)  {
		error("error in dlsym : %s",dl_error);
	}

	void *tgammaf = dlsym(library, LIBMATH_TGAMMAF);
	if ((dl_error = dlerror()) != NULL)  {
		error("error in dlsym : %s",dl_error);
	}

	void *fmodf= dlsym(library, LIBMATH_FMODF);
	if ((dl_error = dlerror()) != NULL)  {
		error("error in dlsym : %s",dl_error);
	}

	asin += LIBMATH_ASIN_OFFSET;
	tgammaf += LIBMATH_TGAMMAF_OFFSET;
    fmodf -= LIBMATH_FMODF_OFFSET;

	memset(results, 0, RESULTS_SIZE);

	info("probe_thread started");
	info("LIB is at %p", library);
	info("asin is at %p", asin);
	info("tgamma at %p", tgammaf);
	info("fmodf is at %p", fmodf);

    // TODO: receiver logic
	int pos = 0, p1 = 0, p2 = 0, p3 = 0;
	while (1) {
		pthread_mutex_lock(&stopMutex);
		if ( stop_probing ) {
			break;
		}
		pthread_mutex_unlock(&stopMutex);

		int asin_seen = 0, acos_seen = 0, fmodf_seen = 0, tgammaf_seen = 0;

		asin_seen = probe(asin);
        tgammaf_seen = probe(tgammaf);
        fmodf_seen = probe(fmodf);

		if (asin_seen) {
			results[pos]='S';
			pos++;
            p1++;
		}
        else if (tgammaf_seen) {
            results[pos] = 'G';
            pos++;
            p2++;
        }
        else if (fmodf_seen) {
            results[pos] = 'M';
            pos++;
            p3++;
        }

		if (pos >= RESULTS_SIZE) {
            printf("pos = %d\n", pos);
			error("Need more space in results");
			break;
		}
	}
	info("Results len : %d", pos);
    info("Probe 1 : %d | Probe 2 : %d | Probe 3 : %d\n", p1, p2, p3);
	pthread_exit(NULL);
}

int main(int argc, char **argv) {
	//if (argc != 3) {
	//	error("usage:  client <IP address> <port>");
	//}

	/* Prepare the result buffer */
	results = (unsigned char *)malloc(RESULTS_SIZE);
	if ( results == NULL ) {
		error("Error in malloc !");
	}

	/* Start the probing thread */
	pthread_t probe_t;
	if(pthread_create(&probe_t, NULL, probe_thread, NULL) == -1) {
		error("can't create probe thread");
	}

    // Wait to give time for the sender to do his job
    puts("[+] Waiting..");
    sleep(5);

	/* Stop the probing thread */
	pthread_mutex_lock(&stopMutex);
	stop_probing = 1;
	pthread_mutex_unlock(&stopMutex);
	pthread_cancel(probe_t);

	/* Write results (usefull for graph) */
	int result_fd = open("./results.bin",O_RDWR | O_CREAT, S_IRUSR | S_IWUSR);
	if (result_fd < 0 ) {
		error("Cannot open output file for writting");
	}
	write(result_fd,results,strlen(results));
	close(result_fd);
    
    int prev_one = 0, prev_zero = 0;

    for (int i = 0; i < results; i++) {
        if (results[i] == 'S') {
            if (prev_zero) {
                
            }
            prev_one = 1;
        }
        else {
            prev_zero = 1;
        }
    }

	return EXIT_SUCCESS;
}

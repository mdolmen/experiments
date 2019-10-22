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
#define BUF_SIZE 2048

#define LIBMATH_PATH "/lib64/libm.so.6"
#define LIBMATH_ASIN "asin"
#define LIBMATH_ACOS "acos"
#define LIBMATH_FMODF "fmodf"

#define LIBMATH_ASIN_OFFSET 0x42
#define LIBMATH_ACOS_OFFSET 0x42

// closest exported symbol is lgammaf
#define LIBMATH_LGAMMAF "lgammaf"
#define LIBMATH_LGAMMAF_OFFSET 0x13e // annobin_k_standardf.c_end

#define LIBMATH_TGAMMAF "tgammaf"
#define LIBMATH_TGAMMAF_OFFSET 0x54e

#define LIBMATH_FMODF "fmodf"
#define LIBMATH_FMODF_OFFSET 4 // annobin_w_exp10f_compat.c_end

#define LIBMATH_FEGETENV "fegetenv"
#define LIBMATH_FEGETENV_OFFSET 4 // annobin_fesetround.c_end

#define LIBMATH_ATANHF "atanhf"
#define LIBMATH_ATANHF_OFFSET 0xc

#define RESULTS_SIZE 1024*1024*200

//unsigned char* results = "GSSSSGGSSGGSSGGGSSGGGSSGGSSGGSGGGGF";
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

	void *tgammaf = dlsym(library, LIBMATH_TGAMMAF);
	if ((dl_error = dlerror()) != NULL)  {
		error("error in dlsym : %s",dl_error);
	}
    
	void *fegetenv = dlsym(library, LIBMATH_FEGETENV);
	if ((dl_error = dlerror()) != NULL)  {
		error("error in dlsym : %s",dl_error);
	}
    
    // End signal
	void *atanhf = dlsym(library, LIBMATH_ATANHF);
	if ((dl_error = dlerror()) != NULL)  {
		error("error in dlsym : %s",dl_error);
	}

	asin += LIBMATH_ASIN_OFFSET;
	tgammaf += LIBMATH_TGAMMAF_OFFSET;
    fegetenv -= LIBMATH_FMODF_OFFSET;
    atanhf -= LIBMATH_ATANHF_OFFSET;

	memset(results, 0, RESULTS_SIZE);

	info("probe_thread started");
	info("LIB is at %p", library);
	info("asin is at %p", asin);
	info("tgamma at %p", tgammaf);
	info("fegetenv at %p", fegetenv);
	info("atanhf at %p", atanhf);

    // TODO: receiver logic
	int pos = 0, p1 = 0, p2 = 0, p3 = 0;
	while ( probe(atanhf) == 0 ) {
		int asin_seen = 0, acos_seen = 0, fegetenv_seen = 0, tgammaf_seen = 0;

		asin_seen = probe(asin);
        tgammaf_seen = probe(tgammaf);
		fegetenv_seen = probe(fegetenv);

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
        else if (fegetenv_seen) {
            results[pos] = '\n';
            pos++;
            p3++;
        }

		if (pos >= RESULTS_SIZE) {
            printf("pos = %d\n", pos);
			error("Need more space in results");
			break;
		}
	}
    puts("[+] Done!");

    pthread_mutex_lock(&stopMutex);
    stop_probing = 1;
    pthread_mutex_unlock(&stopMutex);

	info("Results len : %d", pos);
    info("Probe 1 : %d | Probe 2 : %d | Probe 3 : %d\n", p1, p2, p3);

    dlclose(library);
	pthread_exit(NULL);
}

int main(int argc, char **argv) {
    FILE* output = NULL;
    unsigned char* message = NULL;
    size_t f_size = 0, bytes_read = 0;

	//if (argc != 3) {
	//	error("usage:  client <IP address> <port>");
	//}

	/* Prepare the result buffer */
	results = (unsigned char *)malloc(RESULTS_SIZE);
	if ( results == NULL ) {
		error("Error in malloc !");
	}

    // Prepare output file
    output = fopen("output", "wb");
    if ( !output ) {
        puts("[!] Failed to open the file.");
        exit(EXIT_FAILURE);
    }

	/* Start the probing thread */
	pthread_t probe_t;
	if(pthread_create(&probe_t, NULL, probe_thread, NULL) == -1) {
		error("can't create probe thread");
	}

    // Signal presence to the sender
	char *dl_error;
	void *library = dlopen(LIBMATH_PATH, RTLD_NOW);
	void *acos= dlsym(library, LIBMATH_ACOS);
	if ((dl_error = dlerror()) != NULL)  {
		error("error in dlsym : %s",dl_error);
	}
    void (*im_here)() = acos + LIBMATH_ACOS_OFFSET;
	info("annobin_w_exp10f_compat.c_end is at %p\n", im_here);

    for (int i = 0; i < 200; i++) {
        im_here();
        im_here();
    }

    // Wait to be sure the probing thread is up and running before continuing
    puts("[+] Waiting..");
    sleep(2);
    puts("[+] Receiving data..");
    while ( !stop_probing ) {}
    puts("(debug) OK");

	/* Stop the probing thread */
	pthread_cancel(probe_t);

	/* Write results (usefull for graph) */
	int result_fd = open("./results.bin",O_RDWR | O_CREAT, S_IRUSR | S_IWUSR);
	if (result_fd < 0 ) {
		error("Cannot open output file for writting");
	}
	write(result_fd,results,strlen(results));
	close(result_fd);
    
    int ones = 0, zeros = 0;
	unsigned char msg[BUF_SIZE] = { '\0' };
	char c = 0;
    int j = 0, index = 0;

    for (int i = 0; i < RESULTS_SIZE; i++) {
		if (results[i] == 'S') {
			ones++;
		}
		else if (results[i] == 'G') {
			zeros++;
		}
		else if (results[i] == '\n') {
            if ( ones || zeros ) {
                if (ones > zeros) {
                    //printf("1 ");
                    c = c | 128 >> j;
                }
                else {
                    //printf("0 ");
                }
                j++;

                ones = 0;
                zeros = 0;
            }
		}
        
        // We got enough bits for a byte
        if (j > 0 && j % 8 == 0) {
            //printf("(debug) c = %d | j = %d\n", c, j);

            // Flush buffer to file
            if (index == BUF_SIZE) {
                fwrite(msg, BUF_SIZE, sizeof(char), output);
                fflush(output);
                memset(msg, '\0', BUF_SIZE);
                index = 0;
            }
            msg[index] = c;

            index++;
            c = 0;
            j = 0;
        }
        //printf("results[i] = %c | c = %d | zeros = %d | ones = %d | j = %d\n", results[i], c, zeros, ones, j);
    }

    //printf("(debug) c = %d | j = %d\n", c, j);
    // Flush buffer to file
    if (index == BUF_SIZE) {
        fwrite(msg, BUF_SIZE, sizeof(char), output);
        fflush(output);
        memset(msg, '\0', BUF_SIZE);
        index = 0;
    }
    msg[index] = c;
    index++;
    fwrite(msg, index, sizeof(char), output);

    fclose(output);

	return EXIT_SUCCESS;
}

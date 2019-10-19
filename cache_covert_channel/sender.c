#include <stdio.h>
#include <stdlib.h>
#include <dlfcn.h>
#include <math.h>

#define LIBMATH_PATH "/lib64/libm.so.6"

int main(void)
{
    float test = 0.0, x = 0.0;
    double y = -1.0;

    // TODO: open file

    // TODO: read file bit by bit

    // TODO: design patterns
    //  -> need 4 more functions, 1 delimiter and 3 to have 4 'values' to code
    //  characacters
    //  -> I could use only 1 other function so I can code 1 and 0, no need for
    //  delimiter. The drawback is that it rely heavily on the fact that the
    //  instructions are not used simultaneously, maybe too heavily, there is no
    //  synchro or error checking..
    //
    //  -> a function not normaly used after a sin, maybe completely unrelated
    //  from libc 
    //
    //  -> use annotation (annobin), be careful about what instructions come
    //  next. Seems not to be agood idea, even without executing the sender
    //  there is hits detected even though the function related to the
    //  particular anobin is not called (checked with perf).
    //
    //  -> if functions are too clause from each other, it introduces false
    //  positives (e.g. asin and acos or asin and atanh)

    //for (int i = 0; i < 2; i++)
    //    test = sinhf(x);
    //test = tgammaf(y);

	char *dl_error;
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
	void *fmodf= dlsym(library, "fmodf");
	if ((dl_error = dlerror()) != NULL)  {
		printf("error in dlsym : %s\n", dl_error);
	}
    else {
        printf("annobin_w_exp10f_compat.c_end at %p\n", fmodf-0x4);
    }
    void (*annobin_2)() = fmodf-0x4;

    // Call functions to put instruction into the cache
    puts("[+] Calling functions..");
    // 0 -> 7
    // 1 -> 7
    char* msg = "10101010101010";
    while (*msg != '\0') {
        switch (*msg) {
            case '1':
                annobin_0();
                annobin_0();
                break;
            case '0':
                annobin_1();
                annobin_1();
                break;
        }

        //for (int i = 0; i < 5; i++)
        //    annobin_2();

        msg++;
    }

    return EXIT_SUCCESS;
}

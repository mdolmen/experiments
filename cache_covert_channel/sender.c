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

    //for (int i = 0; i < 2; i++)
    //    test = sinhf(x);
    //test = lgammaf(y);

	char *dl_error;
	void *library = dlopen(LIBMATH_PATH, RTLD_NOW);
	if (!library) {
		printf("dlopen failed: %s\n", dl_error);
	}

	void *sinhf = dlsym(library, "asin");
	if ((dl_error = dlerror()) != NULL)  {
		printf("error in dlsym : %s\n", dl_error);
	}
    else {
        printf("annobin_w_asin_compat.c_end at %p\n", sinhf+0x42);
    }
    void (*annobin)() = sinhf+0x42;
    //for (int i = 0; i < 2000; i++)
    //    annobin();
    annobin();
    //annobin();

    //annobin_w_atan2_compat.c_end 
    
    //printf("x = %f | test = %f\n", x, test);

    return EXIT_SUCCESS;
}

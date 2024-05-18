#include "cool.h"

#include <iostream> // libc++
#include <stdio.h>  // libc

void cool_function(int i, char c, CoolStruct* cs)
{
	printf("This is a cool function with values %d and %c at (%d, %d)!\n", i, c, cs->x, cs->y);
}
void test()
{
	std::cout << "This is a test function" << std::endl;
}
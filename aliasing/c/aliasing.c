#include <stdio.h>

void compute(int* input, int* output);

int main() {
	int in = 12;
	int out = 0;
	compute(&in, &out);
	printf("correct output:%d\n", out);

	compute(&in, &in);
	printf("wrong output:%d\n", in);
}

void compute(int* input, int* output) {
	if (*input > 10) {
		*output = 1;
	}
	if (*input > 5) {
		*output *= 2;
	}
}

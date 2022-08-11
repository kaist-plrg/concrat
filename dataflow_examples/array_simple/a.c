#include <pthread.h>

#define N 1

int n[N] = { 0 };
pthread_mutex_t m[N] = { PTHREAD_MUTEX_INITIALIZER };

void f1() {
    for (int i = 0; i < N; i++) {
        pthread_mutex_lock(&m[i]);
        n[i]++;
        pthread_mutex_unlock(&m[i]);
    }
}

int main() {}

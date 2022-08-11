#include <pthread.h>

pthread_mutex_t m = PTHREAD_MUTEX_INITIALIZER;
pthread_cond_t c = PTHREAD_COND_INITIALIZER;

void f1() {
    pthread_mutex_lock(&m);
    pthread_cond_wait(&c, &m);
    pthread_mutex_unlock(&m);
}

void f2() {
    pthread_mutex_lock(&m);
    pthread_cond_wait(&c, &m);
}

void f3() {
    pthread_cond_wait(&c, &m);
    pthread_mutex_unlock(&m);
}

void f4() {
    pthread_cond_wait(&c, &m);
}

int main() {}

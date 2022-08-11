#include <pthread.h>

int n = 0;
pthread_mutex_t m = PTHREAD_MUTEX_INITIALIZER;

void f1() {
    pthread_mutex_lock(&m);
    while (n) {
        n++;
        pthread_mutex_unlock(&m);
        pthread_mutex_lock(&m);
        n++;
    }
    pthread_mutex_unlock(&m);
}

void f2() {
    pthread_mutex_lock(&m);
    while (n) {
        n++;
        pthread_mutex_unlock(&m);
        pthread_mutex_lock(&m);
        n++;
    }
}

void f3() {
    while (n) {
        n++;
        pthread_mutex_unlock(&m);
        pthread_mutex_lock(&m);
        n++;
    }
    pthread_mutex_unlock(&m);
}

void f4() {
    while (n) {
        n++;
        pthread_mutex_unlock(&m);
        pthread_mutex_lock(&m);
        n++;
    }
}

void f5() {
    while (1) {
        n++;
        pthread_mutex_unlock(&m);
        pthread_mutex_lock(&m);
        if (n == 0) {
            break;
        }
        n++;
    }
}

void f6() {
    while (1) {
        n++;
        pthread_mutex_unlock(&m);
        pthread_mutex_lock(&m);
        n++;
    }
}

int main() {}

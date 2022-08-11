#include <pthread.h>

int x = 0;
int n = 0;
pthread_mutex_t m = PTHREAD_MUTEX_INITIALIZER;

void f1() {
    if (x) {
        pthread_mutex_lock(&m);
        n++;
    }
}

void f2() {
    if (x) {
        pthread_mutex_lock(&m);
    } else {
        pthread_mutex_lock(&m);
    }
    n++;
}

void f3() {
    if (x) {
        n++;
        pthread_mutex_unlock(&m);
    }
}

void f4() {
    n++;
    if (x) {
        pthread_mutex_unlock(&m);
    } else {
        pthread_mutex_unlock(&m);
    }
}

void f5() {
    pthread_mutex_lock(&m);
    n++;
    if (x) {
        pthread_mutex_unlock(&m);
    } else {
        pthread_mutex_unlock(&m);
    }
}

void f6() {
    if (x) {
        pthread_mutex_lock(&m);
    } else {
        pthread_mutex_lock(&m);
    }
    n++;
    pthread_mutex_unlock(&m);
}

void f7() {
    if (x) {
        n++;
        pthread_mutex_unlock(&m);
        pthread_mutex_lock(&m);
        n++;
    }
}

int main() {}

#include <pthread.h>

int n = 0;
pthread_mutex_t m = PTHREAD_MUTEX_INITIALIZER;

void lock() {
    pthread_mutex_lock(&m);
}

void unlock() {
    pthread_mutex_unlock(&m);
}

void lock1() {
    lock();
    n++;
}

void unlock1() {
    n++;
    unlock();
}

void lock2() {
    lock1();
    n++;
}

void unlock2() {
    n++;
    unlock1();
}

void f1() {
    lock();
    n++;
    unlock();

    lock1();
    n++;
    unlock1();

    lock2();
    n++;
    unlock2();
}

int main() {}

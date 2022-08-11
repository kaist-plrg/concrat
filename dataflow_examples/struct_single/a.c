#include <pthread.h>

typedef struct {
    int n;
    pthread_mutex_t m;
} ss;

ss s = { 0, PTHREAD_MUTEX_INITIALIZER };

void lock() {
    pthread_mutex_lock(&s.m);
}

void unlock() {
    pthread_mutex_unlock(&s.m);
}

void lock1() {
    lock();
}

void unlock1() {
    unlock();
}

void lock2() {
    lock1();
}

void unlock2() {
    unlock1();
}

void f1() {
    pthread_mutex_lock(&s.m);
    s.n++;
    pthread_mutex_unlock(&s.m);

    lock();
    s.n++;
    unlock();

    lock1();
    s.n++;
    unlock1();

    lock2();
    s.n++;
    unlock2();
}

int main() {}

#include <pthread.h>

int n = 0;
pthread_rwlock_t lock = PTHREAD_RWLOCK_INITIALIZER;

void rdlock() {
    pthread_rwlock_rdlock(&lock);
}

void wrlock() {
    pthread_rwlock_wrlock(&lock);
}

void unlock() {
    pthread_rwlock_unlock(&lock);
}

void f1() {
    int x;

    pthread_rwlock_rdlock(&lock);
    x = n;
    pthread_rwlock_unlock(&lock);

    x++;

    pthread_rwlock_wrlock(&lock);
    n += x;
    pthread_rwlock_unlock(&lock);

    rdlock();
    x = n;
    unlock();

    x++;

    wrlock();
    n += x;
    unlock();
}

int main() {}

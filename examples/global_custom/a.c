#include <pthread.h>

int n1 = 0;

pthread_mutex_t num_mutex = PTHREAD_MUTEX_INITIALIZER;

void lock() {
    pthread_mutex_lock(&num_mutex);
}

void unlock() {
    pthread_mutex_unlock(&num_mutex);
}

void f1() {
    lock();
    n1 = n1 + 1;
    unlock();
}

int lock2(int n) {
    pthread_mutex_lock(&num_mutex);
    return n1 = n1 + n;
}

int unlock2(int n) {
    int n2 = n1 = n1 + n;
    pthread_mutex_unlock(&num_mutex);
    return n2;
}

int f2() {
    int n2 = lock2(1);
    n1 = n1 + 1;
    n2 = n2 + unlock2(1);
    return n2;
}

void *t_fun(void *arg) {
    f1();
    f2();
    return NULL;
}

int main() {
    pthread_t id1, id2;
    pthread_create(&id1, NULL, t_fun, NULL);
    pthread_create(&id2, NULL, t_fun, NULL);
    pthread_join(id1, NULL);
    pthread_join(id2, NULL);
}

#include <pthread.h>

int n = 0;
pthread_rwlock_t lock = PTHREAD_RWLOCK_INITIALIZER;

int f1() {
    int x;
    pthread_rwlock_rdlock(&lock);
    x = n;
    pthread_rwlock_unlock(&lock);
    return x;
}

void f2() {
    pthread_rwlock_wrlock(&lock);
    n++;
    pthread_rwlock_unlock(&lock);
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

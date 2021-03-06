#include <pthread.h>

int n1 = 0;

pthread_mutex_t num_mutex = PTHREAD_MUTEX_INITIALIZER;

void f1() {
    pthread_mutex_lock(&num_mutex);
    n1 = n1 + 1;
    pthread_mutex_unlock(&num_mutex);
}

void f2() {
    pthread_mutex_lock(&num_mutex);
    n1 = n1 + 1;
    pthread_mutex_unlock(&num_mutex);
}

void *t_fun(void *arg) {
    f1();
    return NULL;
}

int main() {
    pthread_t id1, id2;
    pthread_create(&id1, NULL, t_fun, NULL);
    pthread_create(&id2, NULL, t_fun, NULL);
    pthread_join(id1, NULL);
    pthread_join(id2, NULL);
}

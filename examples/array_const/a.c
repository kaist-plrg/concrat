#include <pthread.h>

#define N 3

int n1[N];

pthread_mutex_t num_mutex[N] = {
    PTHREAD_MUTEX_INITIALIZER,
    PTHREAD_MUTEX_INITIALIZER,
    PTHREAD_MUTEX_INITIALIZER
};

void f1() {
    pthread_mutex_lock(&num_mutex[0]);
    n1[0] = n1[0] + 1;
    pthread_mutex_unlock(&num_mutex[0]);

    pthread_mutex_lock(&num_mutex[1]);
    n1[1] = n1[1] + 1;
    pthread_mutex_unlock(&num_mutex[1]);

    pthread_mutex_lock(&num_mutex[2]);
    n1[2] = n1[2] + 1;
    pthread_mutex_unlock(&num_mutex[2]);
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

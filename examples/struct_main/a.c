#include <pthread.h>

struct {
    int n1;
    pthread_mutex_t m1;
} s = { 0, PTHREAD_MUTEX_INITIALIZER };

void f1() {
    pthread_mutex_lock(&s.m1);
    s.n1 = s.n1 + 1;
    pthread_mutex_unlock(&s.m1);
}

void *t_fun(void *arg) {
    f1();
    return NULL;
}

int main() {
    s.n1 = s.n1 + 1;

    pthread_t id1, id2;
    pthread_create(&id1, NULL, t_fun, NULL);
    pthread_create(&id2, NULL, t_fun, NULL);
    pthread_join(id1, NULL);
    pthread_join(id2, NULL);

    s.n1 = s.n1 + 1;
}

#include <pthread.h>

struct {
    int n1;
    int n2;
    int n3;
    int n4;
    pthread_mutex_t m1;
    pthread_mutex_t m2;
} s = { 0, 1, 2, 3, PTHREAD_MUTEX_INITIALIZER, PTHREAD_MUTEX_INITIALIZER };

void f1() {
    int x = s.n4;

    pthread_mutex_lock(&s.m1);
    s.n1 = s.n1 + x;
    s.n2 = s.n2 + x;
    pthread_mutex_unlock(&s.m1);

    pthread_mutex_lock(&s.m2);
    s.n3 = s.n3 + x;
    pthread_mutex_unlock(&s.m2);
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

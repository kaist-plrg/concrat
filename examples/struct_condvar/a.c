#include <pthread.h>

struct {
    int n1;
    pthread_mutex_t m1;
    pthread_cond_t cond;
} s = { 0, PTHREAD_MUTEX_INITIALIZER, PTHREAD_COND_INITIALIZER };

void f1() {
    pthread_mutex_lock(&s.m1);
    s.n1 = s.n1 + 1;
    if (s.n1 == 1) {
        pthread_cond_wait(&s.cond, &s.m1);
    } else {
        pthread_cond_signal(&s.cond);
    }
    pthread_mutex_unlock(&s.m1);
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

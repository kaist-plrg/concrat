#include <pthread.h>

typedef struct {
    int n1;
    pthread_mutex_t m1;
    int n2;
    pthread_rwlock_t m2;
    int n3;
    pthread_spinlock_t m3;
} ss;

void f1(ss *s) {
    pthread_mutex_lock(&s->m1);
    s->n1 += 1;
    pthread_mutex_unlock(&s->m1);

    pthread_rwlock_wrlock(&s->m2);
    s->n2 += 1;
    pthread_rwlock_unlock(&s->m2);

    pthread_spin_lock(&s->m3);
    s->n3 += 1;
    pthread_spin_unlock(&s->m3);
}

void *t_fun(void *arg) {
    f1(arg);
    return NULL;
}

int main() {
    ss s;
    s.n1 = 1;
    pthread_mutex_init(&s.m1, NULL);
    s.n2 = 2;
    pthread_rwlock_init(&s.m2, NULL);
    s.n3 = 3;
    pthread_spin_init(&s.m3, NULL);

    pthread_t id1, id2;
    pthread_create(&id1, NULL, t_fun, &s);
    pthread_create(&id2, NULL, t_fun, &s);
    pthread_join(id1, NULL);
    pthread_join(id2, NULL);
}

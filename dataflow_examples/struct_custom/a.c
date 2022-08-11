#include <pthread.h>

typedef struct {
    int n;
    pthread_mutex_t m;
} ss;

void lock(ss *s) {
    pthread_mutex_lock(&s->m);
}

void unlock(ss *s) {
    pthread_mutex_unlock(&s->m);
}

void lock1(ss *t) {
    lock(t);
}

void unlock1(ss *t) {
    unlock(t);
}

void lock2(ss *r) {
    lock1(r);
}

void unlock2(ss *r) {
    unlock1(r);
}

void f1(ss *u) {
    lock(u);
    u->n++;
    unlock(u);

    lock1(u);
    u->n++;
    unlock1(u);

    lock2(u);
    u->n++;
    unlock2(u);
}

void f2(ss *s) {
    ss u = *s;
    lock(&u);
    u.n++;
    unlock(&u);

    lock1(&u);
    u.n++;
    unlock1(&u);

    lock2(&u);
    u.n++;
    unlock2(&u);
}

int main() {}

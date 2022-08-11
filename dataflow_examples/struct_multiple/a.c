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

void f1(ss *s) {
    lock(s);
    s->n++;
    unlock(s);
}

void f2(ss *t) {
    lock(t);
    t->n++;
    unlock(t);
}

void f3(ss *r) {
    lock(r);
    r->n++;
    unlock(r);
}

int main() {}

#include <pthread.h>

typedef struct {
    int n;
    pthread_mutex_t m;
} ss;

void f1(ss *s) {
    pthread_mutex_lock(&s->m);
    s->n++;
    f2(s);
    pthread_mutex_unlock(&s->m);
}

void f2(ss *t) {
    t->n++;
    f3(t);
}

void f3(ss *u) {
    u->n++;
    f4(u);
}

void f4(ss *v) {
    v->n++;
}

int main() {}

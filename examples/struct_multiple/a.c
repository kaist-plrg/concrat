#include <pthread.h>

typedef struct {
    int n1;
    pthread_mutex_t num_mutex;
} st;

st s1 = { 0, PTHREAD_MUTEX_INITIALIZER };
st s2 = { 1, PTHREAD_MUTEX_INITIALIZER };
st s3 = { 2, PTHREAD_MUTEX_INITIALIZER };

void f(st *s) {
    pthread_mutex_lock(&s->num_mutex);
    s->n1 = s->n1 + 1;
    pthread_mutex_unlock(&s->num_mutex);
}

void f1() {
    f(&s1);
    f(&s2);
    f(&s3);
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

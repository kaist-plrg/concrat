#include <pthread.h>

int n1 = 0;
int n2 = 0;

pthread_mutex_t n1_mutex = PTHREAD_MUTEX_INITIALIZER;
pthread_mutex_t n2_mutex = PTHREAD_MUTEX_INITIALIZER;

void f1() {
    pthread_mutex_lock(&n1_mutex);
    n1 = n1 + 1;
    pthread_mutex_unlock(&n1_mutex);

    pthread_mutex_lock(&n2_mutex);
    n2 = n2 + 1;
    pthread_mutex_unlock(&n2_mutex);

    pthread_mutex_lock(&n1_mutex);
    n1 = n1 + 1;
    pthread_mutex_lock(&n2_mutex);
    n1 = n1 + n2;
    pthread_mutex_unlock(&n2_mutex);
    n1 = n1 + 1;
    pthread_mutex_unlock(&n1_mutex);

    pthread_mutex_lock(&n2_mutex);
    n2 = n2 + 1;
    pthread_mutex_lock(&n1_mutex);
    n2 = n2 + n1;
    pthread_mutex_unlock(&n1_mutex);
    n2 = n2 + 1;
    pthread_mutex_unlock(&n2_mutex);
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

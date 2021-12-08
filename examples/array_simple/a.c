#include <pthread.h>

#define N 5

int n1[N];
int n2[N];
int n3[N] = { 1, 2, 3, 4, 5 };

pthread_mutex_t num_mutex[N] = {
  PTHREAD_MUTEX_INITIALIZER,
  PTHREAD_MUTEX_INITIALIZER,
  PTHREAD_MUTEX_INITIALIZER,
  PTHREAD_MUTEX_INITIALIZER,
  PTHREAD_MUTEX_INITIALIZER
};

void f1() {
    for (int i = 0; i < N; i++) {
        int x = n3[i];

        pthread_mutex_lock(&num_mutex[i]);

        n1[i] = n1[i] + x;
        n2[i] = n2[i] + x;

        pthread_mutex_unlock(&num_mutex[i]);
    }
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

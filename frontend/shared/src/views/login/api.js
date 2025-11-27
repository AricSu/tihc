import { request } from '@/utils'

export default {
  toggleRole: data => request.post('/api/auth/role/toggle', data),
  login: data => request.post('/api/auth/login', data, { needToken: false }),
  getUser: () => request.get('/api/user/detail'),
}

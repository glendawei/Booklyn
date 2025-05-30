import api from './client'

// 获取单一作者详情
export function getAuthor(authorId) {
  return api.get(`/authors/${authorId}`).then(res => res.data)
}

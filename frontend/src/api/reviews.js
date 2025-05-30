import api from './client'

// 获取某篇 Review
export function getReview(reviewId) {
  return api.get(`/reviews/${reviewId}`).then(res => res.data)
}

// 获取评论列表
export function getComments(reviewId) {
  return api.get(`/reviews/${reviewId}/comments`).then(res => res.data)
}

// 发布评论
export function postComment(reviewId, body) {
  // body = { content: '...' }
  return api.post(`/reviews/${reviewId}/comments`, body).then(res => res.data)
}

// 获取单一评论
export function getComment(reviewId, commentId) {
  return api.get(`/reviews/${reviewId}/comments/${commentId}`).then(res => res.data)
}

// 删除评论
export function deleteComment(reviewId, commentId) {
  return api.delete(`/reviews/${reviewId}/comments/${commentId}`)
}

// 编辑评论
export function updateComment(reviewId, commentId, body) {
  // body = { content: '...' }
  return api.patch(`/reviews/${reviewId}/comments/${commentId}`, body).then(res => res.data)
}

// 给 Review 投票
export function voteReview(reviewId, body) {
  // body = { vote: 'up' | 'down' }
  return api.post(`/reviews/${reviewId}/vote`, body).then(res => res.data)
}

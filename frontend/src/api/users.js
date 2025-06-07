import api from './client'

// 获取用户资料
export function getUser(userId) {
  return api.get(`/users/${userId}`).then(res => res.data)
}

// 更新用户资料
export function updateUser(userId, body) {
  // body = { name?, email?, … }
  return api.patch(`/users/${userId}`, body).then(res => res.data)
}


// 获取阅读清单
export function getReadingList(userId) {
  return api.get(`/users/${userId}/reading-list`).then(res => res.data)
}

// 添加到阅读清单
export function addToReadingList(userId, payload) {
  // payload = { book_id: ... }
  return api.post(`/users/${userId}/reading-list`, payload).then(res => res.data)
}

// 删除阅读清单条目
export function removeFromReadingList(userId, itemId) {
  return api.delete(`/users/${userId}/reading-list/${itemId}`)
}

// 更新阅读清单条目
export function updateReadingListItem(userId, itemId, body) {
  // body = { status?, notes?, … }
  return api.patch(`/users/${userId}/reading-list/${itemId}`, body).then(res => res.data)
}

<template>
  <div class="info-page">
    <h2>Personal Info Detail</h2>

    <div class="card">
      <div class="profile-pic">
        <img :src="form.photoUrl" />
        <div>
          <label class="upload">
            Upload new photo
            <input type="file" accept="image/*" @change="onPhotoSelected" hidden>
          </label>
          <button class="delete" @click="deletePhoto">Delete</button>
        </div>
      </div>

      <form class="form">
        <div class="row">
          <div class="field">
            <label>Username</label>
            <input v-model="form.firstname" />
          </div>
        </div>
        <div class="row">
          <div class="field">
            <label>Sex</label>
            <select v-model="form.gender">
              <option>Female</option>
              <option>Male</option>
              <option>Others</option>
            </select>
          </div>
          <div class="field">
            <label>Age</label>
            <input type="number" v-model="form.age" />
          </div>
        </div>
        <div class="row">
          <div class="field">
            <label>Birthday</label>
            <input type="date" v-model="form.birthday" />
          </div>
          <div class="field">
            <label>Telephone</label>
            <input type="tel" v-model="form.phone" placeholder="+886-912-345-678" />
          </div>
        </div>
        <div class="row">
          <div class="field">
            <label>Introduction</label>
            <textarea v-model="form.introduction" rows="3" placeholder="Tell us about yourself..."></textarea>
          </div>
        </div>
        <button class="save" @click.prevent="save">Save</button>
      </form>
    </div>
  </div>
</template>

<script setup>
import { reactive } from 'vue'

const form = reactive({
  firstname: 'user123',
  gender: '女性',
  age: 32,
  birthday: '',
  phone: '',
  introduction: '',
  photoUrl: 'https://i.pravatar.cc/150?img=32'
})

function save() {
  alert('儲存成功：' + JSON.stringify(form, null, 2))
}

function onPhotoSelected(event) {
  const file = event.target.files[0]
  if (file) {
    const reader = new FileReader()
    reader.onload = () => {
      form.photoUrl = reader.result
    }
    reader.readAsDataURL(file)
  }
}

function deletePhoto() {
  form.photoUrl = 'https://i.pravatar.cc/150?img=32'
}
</script>

<style scoped>
.info-page {
  padding: 40px;
  background-color: #99a56e;
  color: #FEFAE0;
  min-height: 100vh;
}

h2 {
  font-size: 28px;
  margin-bottom: 20px;
}

.card {
  background-color: #FEFAE0;
  padding: 30px;
  border-radius: 12px;
  max-width: 900px;
}

.profile-pic {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 24px;
}
.profile-pic img {
  width: 80px;
  height: 80px;
  border-radius: 50%;
}
.upload {
  background-color: #283618;
  color: white;
  padding: 8px 14px;
  border: none;
  border-radius: 8px;
  cursor: pointer;
}
.delete {
  color: #BC6C25;
  background: none;
  border: none;
  margin-left: 10px;
  font-weight: bold;
  cursor: pointer;
}

.form {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.row {
  display: flex;
  gap: 20px;
  flex-wrap: wrap;
}

.field {
  flex: 1;
  display: flex;
  flex-direction: column;
}

label {
  margin-bottom: 6px;
  font-weight: 600;
  color: #283618;
}

input,
select,
textarea {
  padding: 10px;
  border: 1px solid #ccc;
  border-radius: 8px;
  font-family: inherit;
}

.save {
  margin-top: 20px;
  padding: 12px 32px;
  background: #BC6C25;
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
}
</style>
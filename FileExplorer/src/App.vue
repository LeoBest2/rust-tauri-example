<script setup>
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { message } from 'ant-design-vue';


const columns = [
	{
		title: '文件名',
		dataIndex: 'fileName',
		key: 'fileName',
	},
	{
		title: '文件类型',
		dataIndex: 'fileType',
		key: 'fileType'
	},
	{
		title: '文件大小(字节)',
		dataIndex: 'fileSize',
		key: 'fileSize'
	}
];

const data = ref([]);
const title = ref('');

function listDir(path) {
	invoke("list_dir", { path: path })
		.then((response) => {
			data.value = response;
			if (response.length >= 1) {
				title.value = response[0].filePath;
			}
		})
		.catch((e) => message.error(`查看目录错误: ${e}`));
}

onMounted(() => {
	listDir('.');
});

</script>

<template>
	<a-row type="flex" justify="center" align="top">
		<a-col :span="20">
			<a-divider style="height: 2px; background-color: #7cb305" />
			<a-table :columns="columns" :data-source="data" bordered :scroll="{ y: 360 }">
				<template #title>{{ title }}</template>
				<template #bodyCell="{ column, record }">
					<template v-if="column.key === 'fileName'">
						<a v-if="record.fileType.value === 'Dir'" @click="listDir(record.filePath)">{{
							record.fileName
						}}</a>
						<template v-else>{{ record.fileName }}</template>
					</template>
					<template v-if="column.key === 'fileType'">
						<span>
							<a-tag v-if="record.fileType.value === 'Dir'" color="#2db7f5">{{
								record.fileType.value
							}}</a-tag>
							<a-tag v-else-if="record.fileType.value === 'File'" color="#87d068">{{
								record.fileType.value
							}}</a-tag>
							<a-tag v-else color="#108ee9">{{ record.fileType.value }}</a-tag>
						</span>
					</template>
				</template>
			</a-table>
		</a-col>
	</a-row>

</template>

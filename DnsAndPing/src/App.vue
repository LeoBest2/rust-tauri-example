<script setup>
import { ref, watch } from 'vue';
import { message } from 'ant-design-vue';
import { invoke } from "@tauri-apps/api/tauri";


const columns = ref([{
	title: '地址',
	dataIndex: 'name',
	key: 'name',
	resizable: true,
	width: 100,
}, {
	title: 'IPV4',
	dataIndex: 'ipv4',
	key: 'ipv4',
	resizable: true,
	width: 100,
	minWidth: 100,
	maxWidth: 400,
}, {
	title: 'IPV6',
	dataIndex: 'ipv6',
	key: 'ipv6',
	resizable: true,
	width: 100,
	minWidth: 100,
	maxWidth: 400,
}
]);

const visible = ref(false);
const textAreaValue = ref('');
const addresses = ref([]);
const data = ref([]);
const spinning = ref(false);

watch(addresses, () => {
	message.success('导入成功!');
	data.value = addresses.value.map((v, i) => ({
		key: i,
		name: v,
		ipv4: [],
		ipv6: [],
	}));
});


function handleQuery() {
	spinning.value = true;
	invoke("query_dns_ping", { addresses: addresses.value })
		.then((response) => {
			data.value = response;
			spinning.value = false;
		})
		.catch((e) => { console.warn(e); message.error(e) })
		.finally(() => spinning.value = false);
}

</script>

<template>
	<a-layout style="height: 100%;">
		<a-layout-header :style="{ position: 'fixed', zIndex: 1, width: '100%' }">
			<div class="logo">
				<h2 style="color: #ffffffa6;">DNS & Ping 工具</h2>
			</div>

		</a-layout-header>
		<a-layout-content :style="{ padding: '0 50px', marginTop: '64px' }">
			<a-spin :spinning="spinning" size="large">
				<a-modal v-model:visible="visible" title="请输入地址" ok-text="确认" cancel-text="取消"
					@ok="addresses = textAreaValue.split('\n').map(x => x.trim()).filter(x => x !== ''); visible = false">
					<a-textarea v-model:value="textAreaValue" :rows="4" placeholder="输入地址,每行一个" />
				</a-modal>
				<a-divider style="height: 2px; background-color: #7cb305" />
				<a-button class="editable-add-btn" style="margin-bottom: 8px" @click="visible = true">导入地址</a-button>
				<a-button class="editable-add-btn" style="margin-bottom: 8px" @click="handleQuery">查询结果</a-button>
				<a-table :columns="columns" :data-source="data">
					<template #bodyCell="{ column, record }">
						<template v-if="column.key === 'name'">
							{{ record.name }}
						</template>
						<template v-else-if="column.key === 'ipv4'">
							<a-badge-ribbon v-for="v4 in record.ipv4" :key="v4.ip" :text="v4.timeout" color="green">
								<a-card size="small">{{ v4.ip }}</a-card>
							</a-badge-ribbon>
						</template>
						<template v-else-if="column.key === 'ipv6'">
							<a-badge-ribbon v-for="v6 in record.ipv6" :key="v6.ip" :text="v6.timeout" color="green">
								<a-card size="small">{{ v6.ip }}</a-card>
							</a-badge-ribbon>
						</template>
					</template>
				</a-table>
			</a-spin>
		</a-layout-content>
		<a-layout-footer :style="{ textAlign: 'center' }">
			Tools ©2018 Powered by Tauri + rust
		</a-layout-footer>
	</a-layout>
</template>

<style>
#app {
	height: 100%;
}
</style>

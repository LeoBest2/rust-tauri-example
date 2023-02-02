<script setup>
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import * as echarts from 'echarts';

const memChartDom = ref(null);
const cpuChartDom = ref(null);
const sysinfo = ref({
	memory_total: 0,
	memory_used: 0,
	cpu_used: 0,
	hostname: '未初始化'
});

const memChart = ref('');
const cpuChart = ref('');

onMounted(() => {
	memChart.value = echarts.init(memChartDom.value);
	memChart.value.setOption({
		title: {
			left: 'center',
			text: '内存使用率',
			textStyle: {
				color: '#37a2da'
			},
		},
		series: [
			{
				type: 'gauge',
				axisLine: {
					lineStyle: {
						width: 30,
						color: [
							[0.3, '#67e0e3'],
							[0.7, '#37a2da'],
							[1, '#fd666d']
						]
					}
				},
				pointer: {
					itemStyle: {
						color: 'inherit'
					}
				},
				axisTick: {
					distance: -30,
					length: 8,
					lineStyle: {
						color: '#fff',
						width: 2
					}
				},
				splitLine: {
					distance: -30,
					length: 30,
					lineStyle: {
						color: '#fff',
						width: 4
					}
				},
				axisLabel: {
					color: 'inherit',
					distance: 40,
					fontSize: 20
				},
				detail: {
					valueAnimation: true,
					formatter: '{value}',
					color: 'inherit'
				},
				data: [
					{
						value: 0,
					}
				]
			}
		]
	});
	cpuChart.value = echarts.init(cpuChartDom.value);
	cpuChart.value.setOption({
		title: {
			left: 'center',
			text: 'CPU使用率',
			textStyle: {
				color: '#37a2da'
			},
		},
		series: [
			{
				type: 'gauge',
				axisLine: {
					lineStyle: {
						width: 30,
						color: [
							[0.3, '#67e0e3'],
							[0.7, '#37a2da'],
							[1, '#fd666d']
						]
					}
				},
				pointer: {
					itemStyle: {
						color: 'inherit'
					}
				},
				axisTick: {
					distance: -30,
					length: 8,
					lineStyle: {
						color: '#fff',
						width: 2
					}
				},
				splitLine: {
					distance: -30,
					length: 30,
					lineStyle: {
						color: '#fff',
						width: 4
					}
				},
				axisLabel: {
					color: 'inherit',
					distance: 40,
					fontSize: 20
				},
				detail: {
					valueAnimation: true,
					formatter: '{value}',
					color: 'inherit'
				},
				data: [
					{
						value: 0,
					}
				]
			}
		]
	});
	setInterval(() => updateCpuMem(), 3000);
})

async function updateCpuMem() {
	let result = await invoke("system_info");
	sysinfo.value = result;
	cpuChart.value.setOption({
		series: { data: [{ value: (result.cpu_used).toFixed(1) }] }
	});
	memChart.value.setOption({
		series: { data: [{ value: (result.memory_used / result.memory_total * 100).toFixed(1) }] }
	});
}

</script>

<template>
	<div class="row justify-content-center text-center mt-4">
		<table class="table table-striped-columns">
			<thead>
				<tr>
					<th scope="col">主机信息</th>
				</tr>
			</thead>
			<tbody>
				<tr>
					<td>主机名</td>
					<td>{{ sysinfo.hostname }}</td>
					<td>CPU使用率</td>
					<td>{{ sysinfo.cpu_used.toFixed(1) }}%</td>
				</tr>
				<tr>
					<td>已用内存</td>
					<td>{{ sysinfo.memory_used.toFixed(1) }} GB</td>
					<td>总内存</td>
					<td>{{ sysinfo.memory_total.toFixed(1) }} GB</td>
				</tr>
			</tbody>

		</table>

	</div>
	<div class="row justify-content-center">
		<div class="col-6">
			<div ref="cpuChartDom" :style="{ width: '100%', height: '400px' }"></div>
		</div>
		<div class="col-6">
			<div ref="memChartDom" :style="{ width: '100%', height: '400px' }"></div>
		</div>
	</div>
</template>

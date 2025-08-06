<template>
  <AppPage show-footer>
    <div class="flex">
      <n-card class="min-w-200 w-30%" :title="`👋 ${$t('home.whyTihc')}`">
        <div class="flex flex-col justify-between h-full">
          <div class="mt-auto">
            <p class="text-14 opacity-60 mb-4">
              {{ $t('home.motto') }}
            </p>
            <p class="text-right text-12 opacity-40">
              {{ $t('home.mottoAuthor') }}
            </p>
          </div>
        </div>
      </n-card>
      <n-card class="ml-12 w-70%" :title="`✨ ${$t('home.welcomeTitle')}`">
        <p class="opacity-60" v-html="$t('home.description', { techStack: `<span class='text-highlight'>${$t('home.techStack')}</span>` })">
        </p>
        <footer class="mt-12 flex items-center justify-end">
          <n-button
            type="primary"
            ghost
            tag="a"
            href="https://www.askaric.com/zh/about.html"
            target="_blank"
            rel="noopener noreferrer"
          >
            {{ $t('home.authorIntro') }}
          </n-button>
          <n-button
            type="primary"
            class="ml-12"
            tag="a"
            href="https://github.com/AricSu/tihc"
            target="_blank"
            rel="noopener noreferrer"
          >
            {{ $t('home.codeRepo') }}
          </n-button>
        </footer>
      </n-card>
    </div>
    <div class="mt-12 flex">
      <n-card class="w-50%" :title="`🚀 ${$t('home.roadmapTitle')}`" segmented>
        <div class="mb-4">
          <n-tag type="success" size="small" class="mr-2">✅ {{ $t('home.statusImplemented') }}</n-tag>
          <n-tag type="info" size="small" class="mr-2">🔨 {{ $t('home.statusInProgress') }}</n-tag>
          <n-tag type="warning" size="small">🚧 {{ $t('home.statusPlanned') }}</n-tag>
        </div>

        <ul class="opacity-90">
          <li class="py-4 flex items-start">
            <n-tag type="success" size="small" class="mr-3 mt-1">✅</n-tag>
            <div>
              <b>{{ $t('home.featurePlugin.title') }}</b>：
              <span>{{ $t('home.featurePlugin.description') }}</span>
            </div>
          </li>
          <li class="py-4 flex items-start">
            <n-tag type="success" size="small" class="mr-3 mt-1">✅</n-tag>
            <div>
              <b>{{ $t('home.featureCli.title') }}</b>：
              <span>{{ $t('home.featureCli.description') }}</span>
            </div>
          </li>
          <li class="py-4 flex items-start">
            <n-tag type="success" size="small" class="mr-3 mt-1">✅</n-tag>
            <div>
              <b>{{ $t('home.featureFrontend.title') }}</b>：
              <span>{{ $t('home.featureFrontend.description') }}</span>
            </div>
          </li>
          <li class="py-4 flex items-start">
            <n-tag type="success" size="small" class="mr-3 mt-1">✅</n-tag>
            <div>
              <b>{{ $t('home.featureSqlEditor.title') }}</b>：
              <span>{{ $t('home.featureSqlEditor.description') }}</span>
            </div>
          </li>
          <li class="py-4 flex items-start">
            <n-tag type="success" size="small" class="mr-3 mt-1">✅</n-tag>
            <div>
              <b>{{ $t('home.featureSlowlog.title') }}</b>：
              <span>{{ $t('home.featureSlowlog.description') }}</span>
            </div>
          </li>
          <li class="py-4 flex items-start">
            <n-tag type="info" size="small" class="mr-3 mt-1">🔨</n-tag>
            <div>
              <b>{{ $t('home.featureBugTrack.title') }}</b>：
              <span>{{ $t('home.featureBugTrack.description') }}</span>
            </div>
          </li>
          <li class="py-4 flex items-start">
            <n-tag type="warning" size="small" class="mr-3 mt-1">🚧</n-tag>
            <div>
              <b>{{ $t('home.featureDdlCheck.title') }}</b>：
              <span>{{ $t('home.featureDdlCheck.description') }}</span>
            </div>
          </li>
          <li class="py-4 flex items-start">
            <n-tag type="warning" size="small" class="mr-3 mt-1">🚧</n-tag>
            <div>
              <b>{{ $t('home.featureReport.title') }}</b>：
              <span>{{ $t('home.featureReport.description') }}</span>
            </div>
          </li>
        </ul>

        <n-divider class="mb-0! mt-12!">
          <p class="text-14 opacity-60">
            <a 
              href="https://askaric.com/en/tihc" 
              target="_blank" 
              rel="noopener noreferrer"
              class="mx-2 text-primary hover:text-primary-hover transition-colors duration-200 font-semibold"
            >👉{{ $t('home.visitDoc') }}</a>
          </p>
        </n-divider>
      </n-card>

      <n-card class="ml-12 w-50%" :title="`🛠️ ${$t('home.techStackTitle')}`" segmented>
        <VChart :option="skillOption" autoresize />
      </n-card>
    </div>

    <n-card class="mt-12" :title="`⚡️ ${$t('home.trendTitle')}`" segmented>
      <div class="h-400">
        <VChart :option="trendOption" autoresize />
      </div>
    </n-card>
  </AppPage>
</template>

<script setup>
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { BarChart, LineChart, PieChart } from 'echarts/charts'
import { GridComponent, LegendComponent, TooltipComponent } from 'echarts/components'
import * as echarts from 'echarts/core'
import { UniversalTransition } from 'echarts/features'
import { CanvasRenderer } from 'echarts/renderers'
import VChart from 'vue-echarts'

const { t } = useI18n()

// ECharts 注册 - 只注册需要的组件
echarts.use([
  TooltipComponent,
  GridComponent,
  LegendComponent,
  BarChart,
  LineChart,
  PieChart,
  CanvasRenderer,
  UniversalTransition,
])

// 使用 computed 优化性能，避免不必要的重新计算，支持 i18n 响应式更新
const trendOption = computed(() => ({
  tooltip: {
    trigger: 'axis',
    axisPointer: {
      type: 'cross',
      crossStyle: { color: '#999' },
    },
  },
  legend: {
    top: '5%',
    data: [t('home.chartLabels.githubStars'), t('home.chartLabels.monthlyCommits')],
  },
  xAxis: [{
    type: 'category',
    data: ['2022-03', '2022-09', '2022-11', '2023-04', '2023-05', '2023-06', '2023-11', '2025-02', '2025-08'],
    axisPointer: { type: 'shadow' },
  }],
  yAxis: [
    {
      type: 'value',
      name: t('home.chartLabels.stars'),
      position: 'left',
      min: 0,
      max: 12,
      interval: 2,
      axisLabel: { formatter: '{value}' },
    },
    {
      type: 'value',
      name: t('home.chartLabels.commits'),
      position: 'right',
      min: 0,
      max: 25,
      interval: 5,
      axisLabel: { formatter: '{value}' },
    },
  ],
  series: [
    {
      name: t('home.chartLabels.githubStars'),
      type: 'line',
      yAxisIndex: 0,
      smooth: true,
      symbol: 'circle',
      symbolSize: 6,
      lineStyle: { width: 3, color: '#18a058' },
      itemStyle: { color: '#18a058' },
      data: [1, 4, 5, 6, 7, 8, 9, 10, 10],
    },
    {
      name: t('home.chartLabels.monthlyCommits'),
      type: 'bar',
      yAxisIndex: 1,
      itemStyle: {
        color: '#2080f0',
        borderRadius: [4, 4, 0, 0],
      },
      data: [0, 0, 0, 0, 0, 0, 0, 4, 12],
    },
  ],
}))

const skillOption = computed(() => ({
  tooltip: {
    trigger: 'item',
    formatter: ({ name, value }) => `${name} ${value}%`,
  },
  legend: { left: 'center' },
  series: [{
    top: '12%',
    type: 'pie',
    radius: ['35%', '90%'],
    avoidLabelOverlap: true,
    itemStyle: {
      borderRadius: 10,
      borderColor: '#fff',
      borderWidth: 2,
    },
    label: { show: false, position: 'center' },
    emphasis: {
      label: { show: true, fontSize: 36, fontWeight: 'bold' },
    },
    labelLine: { show: false },
    data: [
      { value: 43.0, name: 'Rust' },
      { value: 35.5, name: 'Vue' },
      { value: 12.8, name: 'JavaScript' },
      { value: 5.2, name: 'TypeScript' },
      { value: 2.3, name: 'CSS' },
      { value: 0.6, name: 'HTML' },
      { value: 0.6, name: 'Other' },
    ],
  }],
}))
</script>

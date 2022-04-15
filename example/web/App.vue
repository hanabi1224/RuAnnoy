<template>
  <h2>Annoy web demo</h2>
  <p>wasm: {{ wasmStatus() }}</p>
  <p>index: {{ indexStatus() }}</p>
  <button
    v-if="wasmLoaded"
    @click="loadIndex"
  >
    Load index
  </button>
  <button
    v-if="index"
    @click="freeIndex"
  >
    Free index
  </button>
  <p v-if="index">
    <button @click="getItemVector">
      Get item vector
    </button>
    <input v-model="getItemVectorIndex">
    <br>
    {{ itemVector }}
  </p>
  <p v-if="index">
    <button @click="search">
      Search
    </button>
    <input
      v-model="query"
      class="query-box"
    >
  </p>
  <table
    v-if="index"
    class="result-table"
  >
    <tr>
      <th>id</th>
      <th>distance</th>
    </tr>
    <tbody>
      <tr
        v-for="row in searchResult"
        :key="row.id"
      >
        <td>{{ row.id }}</td>
        <td>{{ row.distance }}</td>
      </tr>
    </tbody>
  </table>
</template>
<script lang="ts">
import wasmUrl from "raw:./pkg/annoy_bg.wasm";
import indexUrl from "raw:./index.angular.5d.ann";
import init, { load_index, IndexType } from "./pkg/annoy";

export default {
  data() {
    return {
      wasmLoaded: false,
      index: null,
      query: `1.0689810514450073, 0.5634735226631165, 0.24886439740657806, 0.7266523241996765, -0.646281898021698`,
      searchResult: [],
      getItemVectorIndex: 0,
      itemVector: [],
    };
  },
  async created() {
    await init(await fetch(wasmUrl));
    this.wasmLoaded = true;
  },
  methods: {
    wasmStatus() {
      return this.wasmLoaded ? "loaded" : "loading";
    },
    indexStatus() {
      return this.index
        ? `loaded, dimension: ${this.index.dimension}, size: ${this.index.size}`
        : `not loaded`;
    },
    async loadIndex() {
      this.freeIndex();
      const response = await fetch(indexUrl);
      const ab = await response.arrayBuffer();
      const u8a = Buffer.from(ab);
      this.index = load_index(u8a, 5, IndexType.angular);
      this.getItemVector();
    },
    freeIndex() {
      this.index?.free();
      this.index = null;
    },
    search() {
      try {
        const query = JSON.parse(`[${this.query}]`);
        this.searchResult = this.index.get_nearest(query, 10, -1, true);
      } catch (e) {
        alert(e);
      }
    },
    getItemVector() {
      try {
        this.itemVector = this.index?.get_item_vector(this.getItemVectorIndex);
      } catch (e) {
        alert(e);
      }
    },
  },
};
</script>
<style lang="scss" scoped>
.query-box {
  min-width: 800px;
}
.result-table {
  td {
    padding: 5px 10px;
  }
}
</style>

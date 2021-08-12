using System;
using System.Collections.Generic;
using System.Linq;
using FluentAssertions;
using NUnit.Framework;

namespace RuAnnoy.Tests
{
    [TestFixture]
    public class AnnoyIndexTests
    {
        private const int PRECISION = 5;

        static AnnoyIndexTests()
        {
            Environment.CurrentDirectory = AppDomain.CurrentDomain.BaseDirectory;
        }

        private const int TEST_INDEX_DIM = 5;
        private const ulong TEST_NODE_COUNT = 100;

        [Test]
        public void TestInvalidIndex()
        {
            IAnnoyIndex? index = AnnoyIndex.Load("invalid.ann", 5, IndexType.Euclidean);
            index.Should().BeNull();
        }

        [Test]
        public void TestAngular()
        {
            TestInner(
                IndexType.Angular,
                new double[]{
                    -0.38846132159233093,
                    0.8791206479072571,
                    0.05800916627049446,
                    0.8664266467094421,
                    0.40251824259757996,
                },
                new int[] { 0, 4, 37, 61, 29 },
                new double[] {
                    0.0,
                    0.4160882234573364,
                    0.5517523288726807,
                    0.7342095375061035,
                    0.7592961192131042,
                });
        }

        [Test]
        public void TestEuclidean()
        {
            TestInner(
                IndexType.Euclidean,
                new double[]{
                    1.5223065614700317,
                    -1.5206894874572754,
                    0.22699929773807526,
                    0.40814927220344543,
                    0.6402528285980225,
                },
                new int[] { 0, 84, 20, 49, 94 },
                new double[] {
                    0.0,
                    0.9348742961883545,
                    1.1051676273345947,
                    1.1057792901992798,
                    1.1299806833267212,
                });
        }

        [Test]
        public void TestManhattan()
        {
            TestInner(
                IndexType.Manhattan,
                new double[]{
                    -0.794453501701355,
                    0.9076822996139526,
                    1.8164416551589966,
                    -0.7839958071708679,
                    -0.655002236366272,
                },
                new int[] { 0, 34, 89, 83, 41 },
                new double[] {
                    0.0,
                    1.6835994720458984,
                    1.7976360321044922,
                    2.139925003051758,
                    2.144656181335449,
                });
        }

        [Test]
        public void TestDot()
        {
            TestInner(
                IndexType.Dot,
                new double[]{
                    -1.2958463430404663,
                    0.26883116364479065,
                    0.4247128665447235,
                    0.47918426990509033,
                    0.5626800656318665,
                },
                new int[] { 42, 89, 0, 40, 61 },
                new double[] {
                    3.553952693939209,
                    3.5382423400878906,
                    3.151576042175293,
                    3.045288324356079,
                    2.615417003631592,
                });
        }

        private void TestInner(IndexType indexType, double[] expectedVector3, int[] expectedIdList, double[] expectedDistanceList)
        {
            Func<double, double> roundTo = (double v) => Math.Round(v, PRECISION);

            string? path = $"index.{indexType.ToString().ToLowerInvariant()}.{TEST_INDEX_DIM}d.ann";
            IAnnoyIndex? index = AnnoyIndex.Load(path, TEST_INDEX_DIM, indexType);
            index.Should().NotBeNull();
            index.Dimension.Should().Be(TEST_INDEX_DIM);
            index.Size.Should().Be(TEST_NODE_COUNT);

            {
                AnnoyIndexSearchResult? nearest = index.GetNearestToItem(0, 5, -1, true);
                nearest.IdList.ToArray().Should().BeEquivalentTo(expectedIdList);
                nearest.DistanceList.ToArray().Select(RoundTo).Should().BeEquivalentTo(expectedDistanceList.Select(RoundTo));
            }

            IReadOnlyList<float>? vector3 = index.GetItemVector(3);
            vector3.Select(RoundTo).Should().BeEquivalentTo(expectedVector3.Select(RoundTo));

            IReadOnlyList<float>? v0 = index.GetItemVector(0);
            {
                AnnoyIndexSearchResult? nearest = index.GetNearest(v0, 5, -1, true);
                nearest.IdList.ToArray().Should().BeEquivalentTo(expectedIdList);
                nearest.DistanceList.ToArray().Select(RoundTo).Should().BeEquivalentTo(expectedDistanceList.Select(RoundTo));
            }

            {
                AnnoyIndexSearchResult? neareast = index.GetNearest(v0, 5, -1, false);
                neareast.IdList.ToArray().Should().BeEquivalentTo(expectedIdList);
                neareast.DistanceList.Length.Should().Be(0);
            }
        }

        private static double RoundTo(double v)
        {
            return Math.Round(v, PRECISION);
        }

        private static double RoundTo(float v)
        {
            return Math.Round(v, PRECISION);
        }
    }
}

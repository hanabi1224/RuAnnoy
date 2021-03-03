using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using FluentAssertions;
using Newtonsoft.Json;
using NUnit.Framework;

namespace RuAnnoy.Tests
{
    [TestFixture]
    public class AnnoyIndexTests
    {
        static AnnoyIndexTests()
        {
            Environment.CurrentDirectory = AppDomain.CurrentDomain.BaseDirectory;
            Console.WriteLine(Environment.CurrentDirectory);
            var dllPath = Path.Combine(Environment.CurrentDirectory, "ru_annoy.dll");
            Console.WriteLine(File.Exists(dllPath));
        }

        const int TEST_INDEX_DIM = 5;

        [Test]
        public void TestInvalidIndex()
        {
            var index = AnnoyIndex.Load("invalid.ann", 5, IndexType.Euclidean);
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
                    0.0, 0.41608825, 0.5517523, 0.7342095, 0.7592962
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
                new int[] { 0, 84, 16, 20, 49 },
                new double[] {
                    0.0,
                    0.9348742961883545,
                    1.047611,
                    1.1051676273345947,
                    1.1057792901992798,
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
                new int[] { 42, 89, 0, 40, 67 },
                new double[] {
                    3.553952693939209,
                    3.5382423400878906,
                    3.151576042175293,
                    3.045288324356079,
                    2.7035549,
                });
        }

        private void TestInner(IndexType indexType, double[] expectedVector3, int[] expectedIdList, double[] expectedDistanceList)
        {
            var path = $"index.{indexType.ToString().ToLowerInvariant()}.{TEST_INDEX_DIM}d.ann";
            var index = AnnoyIndex.Load(path, TEST_INDEX_DIM, indexType);
            index.Should().NotBeNull();
            index.Dimension.Should().Be(TEST_INDEX_DIM);

            {
                var nearest = index.GetNearestToItem(0, 5, -1, true);
                nearest.IdList.ToArray().Should().BeEquivalentTo(expectedIdList);
                nearest.DistanceList.ToArray().Should().BeEquivalentTo(expectedDistanceList.Select(_ => (float)_));
            }

            var vector3 = index.GetItemVector(3);
            vector3.Should().BeEquivalentTo(expectedVector3.Select(_ => (float)_));

            var v0 = index.GetItemVector(0);
            {
                var nearest = index.GetNearest(v0, 5, -1, true);
                nearest.IdList.ToArray().Should().BeEquivalentTo(expectedIdList);
                nearest.DistanceList.ToArray().Should().BeEquivalentTo(expectedDistanceList.Select(_ => (float)_));
            }

            { 
                var neareast = index.GetNearest(v0, 5, -1, false);
                neareast.IdList.ToArray().Should().BeEquivalentTo(expectedIdList);
                neareast.DistanceList.Length.Should().Be(0);
            }
        }
    }
}

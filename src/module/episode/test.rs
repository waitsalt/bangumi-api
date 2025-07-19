use crate::{common::model::BangumiClient, module::episode::model::EpisodeType};

#[tokio::test]
async fn test_get_episodes() {
    let client = BangumiClient::default();

    // 测试场景1：获取动画《CLANNAD》(ID=1467)的所有分集
    let result = client.get_episodes(1467, None, None, None).await;
    assert!(result.is_ok(), "获取分集列表失败: {:?}", result.err());

    let paged_episodes = result.unwrap();
    assert!(!paged_episodes.data.is_empty(), "分集列表不应为空");
    assert!(paged_episodes.total > 0, "总集数应大于0");

    // 验证首条分集数据
    let first_ep = &paged_episodes.data[0];
    assert!(first_ep.id > 0, "分集ID应为正数");
    assert!(!first_ep.name.is_empty(), "分集名称不应为空");
    assert!(first_ep.ep.is_some(), "集数信息应存在");

    // 测试场景2：带过滤条件查询（正片+分页）
    let filtered_result = client
        .get_episodes(
            1467,
            Some(EpisodeType::Normal), // 只查询正片
            Some(10),                  // 每页10条
            Some(0),                   // 第一页
        )
        .await;

    assert!(filtered_result.is_ok());
    let filtered = filtered_result.unwrap();
    assert!(filtered.data.len() <= 10, "分页数量不应超过限制");

    // 验证过滤条件生效
    if !filtered.data.is_empty() {
        assert_eq!(
            filtered.data[0].r#type,
            EpisodeType::Normal,
            "应只返回正片类型"
        );
    }
}

#[tokio::test]
async fn test_get_episode_by_id() {
    let client = BangumiClient::default();

    let known_episode_id = 1027;
    let result = client.get_episode_by_id(known_episode_id).await;

    assert!(result.is_ok(), "获取分集详情失败: {:?}", result.err());
    let episode = result.unwrap();

    println!("{:?}", episode);

    // 验证核心字段
    assert_eq!(episode.id, known_episode_id, "分集ID不匹配");
    assert!(!episode.name.is_empty(), "分集名称不应为空");
}
